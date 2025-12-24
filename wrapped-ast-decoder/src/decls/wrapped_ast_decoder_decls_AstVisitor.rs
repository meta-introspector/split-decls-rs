use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Helper struct to traverse the AST and collect statistics
struct AstVisitor {
    stats: AstStatistics,
    #[allow(dead_code)]
    file_path: PathBuf,
}
