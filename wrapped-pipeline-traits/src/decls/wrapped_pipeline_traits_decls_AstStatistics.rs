use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Comprehensive AST analysis data for a Rust project
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AstStatistics {
    pub node_type_counts: HashMap<String, u32>,
    pub variable_declarations: Vec<VariableInfo>,
    pub function_definitions: Vec<FunctionInfo>,
    pub import_statements: Vec<ImportInfo>,
}
