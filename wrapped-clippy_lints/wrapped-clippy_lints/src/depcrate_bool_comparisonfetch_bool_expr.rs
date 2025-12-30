// Generated macro for fetch_bool_expr (function)
macro_rules! Depcrate_bool_comparisonfetch_bool_expr {
() => {
// Module: crate::bool_comparison
// Provides: {"fetch_bool_expr"}
// Dependencies: {}
fn fetch_bool_expr (expr : & Expr < '_ >) -> Option < bool > { if let ExprKind :: Lit (lit_ptr) = peel_blocks (expr) . kind && let LitKind :: Bool (value) = lit_ptr . node { return Some (value) ; } None }
};
}
