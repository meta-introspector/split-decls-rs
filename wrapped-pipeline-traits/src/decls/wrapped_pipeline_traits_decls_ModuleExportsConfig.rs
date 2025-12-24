use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Deserialize, Clone)]
pub struct ModuleExportsConfig {
    pub modules: Option<Vec<String>>,
}
