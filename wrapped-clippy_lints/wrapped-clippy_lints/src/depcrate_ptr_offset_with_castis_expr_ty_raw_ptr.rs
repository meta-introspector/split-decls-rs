// Generated macro for is_expr_ty_raw_ptr (function)
macro_rules! Depcrate_ptr_offset_with_castis_expr_ty_raw_ptr {
() => {
// Module: crate::ptr_offset_with_cast
// Provides: {"is_expr_ty_raw_ptr"}
// Dependencies: {}
fn is_expr_ty_raw_ptr (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { cx . typeck_results () . expr_ty (expr) . is_raw_ptr () }
};
}
