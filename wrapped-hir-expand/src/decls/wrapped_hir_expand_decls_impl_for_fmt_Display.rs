use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl fmt::Display for RenderedExpandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
