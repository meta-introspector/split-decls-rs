// Generated macro for is_cast_between_fixed_and_target (function)
macro_rules! Depcrate_operators_absurd_extreme_comparisonsis_cast_between_fixed_and_target {
() => {
// Module: crate::operators::absurd_extreme_comparisons
// Provides: {"is_cast_between_fixed_and_target"}
// Dependencies: {}
fn is_cast_between_fixed_and_target < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) -> bool { if let ExprKind :: Cast (cast_exp , _) = expr . kind { let precast_ty = cx . typeck_results () . expr_ty (cast_exp) ; let cast_ty = cx . typeck_results () . expr_ty (expr) ; return is_isize_or_usize (precast_ty) != is_isize_or_usize (cast_ty) ; } false }
};
}
