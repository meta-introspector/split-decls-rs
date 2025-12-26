use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl From<Errors> for Result<(), Errors> {
    fn from(e: Errors) -> Self {
        Err(e)
    }
}
