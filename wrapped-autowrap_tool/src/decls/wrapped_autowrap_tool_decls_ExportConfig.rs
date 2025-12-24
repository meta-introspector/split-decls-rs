use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Deserialize)]
pub struct ExportConfig {
    pub target_dir: String,
    pub create_tar: bool,
    pub tar_name: String,
}
