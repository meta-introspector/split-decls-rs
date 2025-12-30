// Generated macro for is_unwrap_call (function)
macro_rules! Depcrate_methods_readonly_write_lockis_unwrap_call {
() => {
// Module: crate::methods::readonly_write_lock
// Provides: {"is_unwrap_call"}
// Dependencies: {}
fn is_unwrap_call (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { if let ExprKind :: MethodCall (path , receiver , [] , _) = expr . kind && path . ident . name == sym :: unwrap { cx . typeck_results () . expr_ty (receiver) . peel_refs () . is_diag_item (cx , sym :: Result) } else { false } }
};
}
