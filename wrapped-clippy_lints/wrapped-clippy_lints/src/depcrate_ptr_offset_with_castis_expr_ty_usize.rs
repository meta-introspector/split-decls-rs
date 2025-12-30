// Generated macro for is_expr_ty_usize (function)
macro_rules! Depcrate_ptr_offset_with_castis_expr_ty_usize {
() => {
// Module: crate::ptr_offset_with_cast
// Provides: {"is_expr_ty_usize"}
// Dependencies: {}
fn is_expr_ty_usize (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { cx . typeck_results () . expr_ty (expr) == cx . tcx . types . usize }
};
}
