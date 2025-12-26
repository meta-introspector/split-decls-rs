use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl fmt::Display for FromOsStrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OsStr contains invalid UTF-8")
    }
}
