// Generated macro for eq_block (function)
macro_rules! Depcrate_ast_utilseq_block {
() => {
// Module: crate::ast_utils
// Provides: {"eq_block"}
// Dependencies: {}
pub fn eq_block (l : & Block , r : & Block) -> bool { l . rules == r . rules && over (& l . stmts , & r . stmts , eq_stmt) }
};
}
