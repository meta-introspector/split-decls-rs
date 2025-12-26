use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct WrappingConfig {
    #[serde(default)]
    pub crates: Vec<String>,
}
