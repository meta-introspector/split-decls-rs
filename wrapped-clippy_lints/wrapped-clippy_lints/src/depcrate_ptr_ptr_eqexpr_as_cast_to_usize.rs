// Generated macro for expr_as_cast_to_usize (function)
macro_rules! Depcrate_ptr_ptr_eqexpr_as_cast_to_usize {
() => {
// Module: crate::ptr::ptr_eq
// Provides: {"expr_as_cast_to_usize"}
// Dependencies: {}
fn expr_as_cast_to_usize < 'tcx > (cx : & LateContext < 'tcx > , cast_expr : & 'tcx Expr < '_ >) -> Option < & 'tcx Expr < 'tcx > > { if ! cast_expr . span . from_expansion () && cx . typeck_results () . expr_ty (cast_expr) == cx . tcx . types . usize && let ExprKind :: Cast (expr , _) = cast_expr . kind { Some (expr) } else { None } }
};
}
