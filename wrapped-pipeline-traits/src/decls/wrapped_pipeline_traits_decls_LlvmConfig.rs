use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Deserialize, Clone)]
pub struct LlvmConfig {
    #[serde(rename = "download-ci-llvm")]
    pub download_ci_llvm: Option<bool>,
    pub ninja: Option<bool>,
}
