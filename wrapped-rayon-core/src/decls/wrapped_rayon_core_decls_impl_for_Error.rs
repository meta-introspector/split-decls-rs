use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Error for ThreadPoolBuildError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::GlobalPoolAlreadyInitialized => GLOBAL_POOL_ALREADY_INITIALIZED,
            ErrorKind::CurrentThreadAlreadyInPool => CURRENT_THREAD_ALREADY_IN_POOL,
            ErrorKind::IOError(ref e) => e.description(),
        }
    }
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            ErrorKind::GlobalPoolAlreadyInitialized
            | ErrorKind::CurrentThreadAlreadyInPool => None,
            ErrorKind::IOError(e) => Some(e),
        }
    }
}
