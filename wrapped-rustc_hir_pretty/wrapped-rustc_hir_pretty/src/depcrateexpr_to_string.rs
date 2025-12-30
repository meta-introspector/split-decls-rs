// Generated macro for expr_to_string (function)
macro_rules! Depcrateexpr_to_string {
() => {
// Module: crate
// Provides: {"expr_to_string"}
// Dependencies: {}
pub fn expr_to_string (ann : & dyn PpAnn , pat : & hir :: Expr < '_ >) -> String { to_string (ann , | s | s . print_expr (pat)) }
};
}
