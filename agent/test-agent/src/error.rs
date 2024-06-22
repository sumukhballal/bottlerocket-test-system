use snafu::Snafu;
use std::fmt::{Debug, Display, Formatter};

/// The `Error` type for the `TestAgent`. Errors originating from the `Client` or the `Runner` are
/// passed through, preserving their type. Errors originating with the `Agent` are of the
/// [`AgentError`] type.
#[derive(Debug)]
pub enum Error<C, R>
where
    C: Debug + Display + Send + Sync + 'static,
    R: Debug + Display + Send + Sync + 'static,
{
    /// An error originating from the [`Agent`].
    Agent(AgentError),
    /// An error originating from the [`Client`].
    Client(C),
    /// An error originating from the [`Runner`].
    Runner(R),
}

/// The `Result` type for the `TestAgent`.
pub type Result<T, C, R> = std::result::Result<T, Error<C, R>>;

impl<C, R> std::error::Error for Error<C, R>
where
    C: Debug + Display + Send + Sync + 'static,
    R: Debug + Display + Send + Sync + 'static,
{
}

impl<C, R> Display for Error<C, R>
where
    C: Debug + Display + Send + Sync + 'static,
    R: Debug + Display + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Agent(e) => write!(f, "agent error: {}", e),
            Error::Client(e) => write!(f, "client error: {}", e),
            Error::Runner(e) => write!(f, "agent error: {}", e),
        }
    }
}

/// An error that has originated with the [`Agent`].
#[derive(Debug, Snafu)]
pub struct AgentError(InnerError);

/// The private error type, [`AgentError'] is opaque. `InnerError` is the underlying error type.
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub(crate) enum InnerError {
    #[snafu(display("An error occurred while creating archive: {}", source))]
    Archive { source: std::io::Error },

    #[snafu(display("Info Client: {}", source))]
    InfoClient { source: InfoClientError },
}

impl<C, R> From<InnerError> for Error<C, R>
where
    C: Debug + Display + Send + Sync + 'static,
    R: Debug + Display + Send + Sync + 'static,
{
    fn from(e: InnerError) -> Self {
        Error::Agent(e.into())
    }
}

pub type InfoClientResult<T> = std::result::Result<T, InfoClientError>;

#[derive(Debug)]
pub enum InfoClientError {
    /// The client could not be created.
    InitializationFailed(Option<Box<dyn std::error::Error + Send + Sync + 'static>>),

    /// A communication with Kubernetes failed.
    RequestFailed(Option<Box<dyn std::error::Error + Send + Sync + 'static>>),
}

impl Display for InfoClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InitializationFailed(e) => write!(f, "initialization failed: {:?}", e),
            Self::RequestFailed(e) => write!(f, "request failed: {:?}", e),
        }
    }
}

impl std::error::Error for InfoClientError {}
