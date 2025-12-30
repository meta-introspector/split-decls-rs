// Generated macro for eq_path (function)
macro_rules! Depcrate_ast_utilseq_path {
() => {
// Module: crate::ast_utils
// Provides: {"eq_path"}
// Dependencies: {}
pub fn eq_path (l : & Path , r : & Path) -> bool { over (& l . segments , & r . segments , eq_path_seg) }
};
}
