use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CrateInfo {
    pub name: &'static str,
    pub path: &'static str,
}