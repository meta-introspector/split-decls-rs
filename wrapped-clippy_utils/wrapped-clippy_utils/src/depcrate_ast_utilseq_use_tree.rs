// Generated macro for eq_use_tree (function)
macro_rules! Depcrate_ast_utilseq_use_tree {
() => {
// Module: crate::ast_utils
// Provides: {"eq_use_tree"}
// Dependencies: {}
pub fn eq_use_tree (l : & UseTree , r : & UseTree) -> bool { eq_path (& l . prefix , & r . prefix) && eq_use_tree_kind (& l . kind , & r . kind) }
};
}
