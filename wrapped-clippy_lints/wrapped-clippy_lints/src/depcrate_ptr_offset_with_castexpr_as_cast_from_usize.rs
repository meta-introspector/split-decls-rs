// Generated macro for expr_as_cast_from_usize (function)
macro_rules! Depcrate_ptr_offset_with_castexpr_as_cast_from_usize {
() => {
// Module: crate::ptr_offset_with_cast
// Provides: {"expr_as_cast_from_usize"}
// Dependencies: {}
fn expr_as_cast_from_usize < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) -> Option < & 'tcx Expr < 'tcx > > { if let ExprKind :: Cast (cast_lhs_expr , _) = expr . kind && is_expr_ty_usize (cx , cast_lhs_expr) { return Some (cast_lhs_expr) ; } None }
};
}
