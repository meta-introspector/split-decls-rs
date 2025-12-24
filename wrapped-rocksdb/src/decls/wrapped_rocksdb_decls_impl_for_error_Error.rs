use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}
