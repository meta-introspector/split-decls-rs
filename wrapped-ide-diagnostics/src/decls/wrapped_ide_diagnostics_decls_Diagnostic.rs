use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug)]
pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub message: String,
    pub range: FileRange,
    pub severity: Severity,
    pub unused: bool,
    pub experimental: bool,
    pub fixes: Option<Vec<Assist>>,
    pub main_node: Option<InFile<SyntaxNodePtr>>,
}
