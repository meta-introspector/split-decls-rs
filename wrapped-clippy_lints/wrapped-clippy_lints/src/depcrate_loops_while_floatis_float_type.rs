// Generated macro for is_float_type (function)
macro_rules! Depcrate_loops_while_floatis_float_type {
() => {
// Module: crate::loops::while_float
// Provides: {"is_float_type"}
// Dependencies: {}
fn is_float_type (cx : & rustc_lint :: LateContext < '_ > , expr : & rustc_hir :: Expr < '_ >) -> bool { cx . typeck_results () . expr_ty (expr) . is_floating_point () }
};
}
