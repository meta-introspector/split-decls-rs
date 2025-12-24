use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl std::error::Error for GetTimezoneError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GetTimezoneError::FailedParsingString => None,
            GetTimezoneError::IoError(err) => Some(err),
            GetTimezoneError::OsError => None,
        }
    }
}
