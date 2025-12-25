use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl fmt::Display for ThreadPoolBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::CurrentThreadAlreadyInPool => {
                CURRENT_THREAD_ALREADY_IN_POOL.fmt(f)
            }
            ErrorKind::GlobalPoolAlreadyInitialized => {
                GLOBAL_POOL_ALREADY_INITIALIZED.fmt(f)
            }
            ErrorKind::IOError(e) => e.fmt(f),
        }
    }
}
