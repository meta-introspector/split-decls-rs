use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimpleCrateInfo {
    pub name: String,
    pub version: String,
    pub manifest_path: PathBuf,
}
