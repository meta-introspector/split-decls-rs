use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Default for Suggestions {
    fn default() -> Self {
        Self::Enabled(vec![])
    }
}
