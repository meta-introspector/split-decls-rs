// Generated macro for check (function)
macro_rules! Depcrate_operators_invalid_upcast_comparisonscheck {
() => {
// Module: crate::operators::invalid_upcast_comparisons
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , cmp : BinOpKind , lhs : & 'tcx Expr < '_ > , rhs : & 'tcx Expr < '_ > , span : Span ,) { let normalized = comparisons :: normalize_comparison (cmp , lhs , rhs) ; let Some ((rel , normalized_lhs , normalized_rhs)) = normalized else { return ; } ; let lhs_bounds = numeric_cast_precast_bounds (cx , normalized_lhs) ; let rhs_bounds = numeric_cast_precast_bounds (cx , normalized_rhs) ; upcast_comparison_bounds_err (cx , span , rel , lhs_bounds , normalized_lhs , normalized_rhs , false) ; upcast_comparison_bounds_err (cx , span , rel , rhs_bounds , normalized_rhs , normalized_lhs , true) ; }
};
}
