use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub struct DeclArgs {
    pub node_type: Option<String>,
    pub name: Option<String>,
    pub vis: Option<String>,
    pub hash: Option<String>,
    pub extra: Vec<(String, String)>,
}
