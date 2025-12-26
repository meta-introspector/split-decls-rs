use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Default for Hasher {
    fn default() -> Self {
        Self::new()
    }
}
