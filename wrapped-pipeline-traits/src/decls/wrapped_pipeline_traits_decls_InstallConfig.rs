use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Deserialize, Clone)]
pub struct InstallConfig {
    pub prefix: Option<String>,
    pub sysconfdir: Option<String>,
}
