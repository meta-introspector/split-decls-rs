use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Information about a function found in the AST
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub name: String,
    pub visibility: String,
    pub arg_count: u32,
    pub arg_types: Vec<String>,
    pub return_type: String,
    pub is_async: bool,
    pub is_unsafe: bool,
    pub is_const: bool,
}
