// Generated macro for impl_3043 (impl)
macro_rules! Depcrate_invalid_upcast_comparisonsimpl_3043 {
() => {
// Module: crate::invalid_upcast_comparisons
// Provides: {"impl_3043"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for InvalidUpcastComparisons { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if let ExprKind :: Binary (ref cmp , lhs , rhs) = expr . kind { let normalized = comparisons :: normalize_comparison (cmp . node , lhs , rhs) ; let Some ((rel , normalized_lhs , normalized_rhs)) = normalized else { return ; } ; let lhs_bounds = numeric_cast_precast_bounds (cx , normalized_lhs) ; let rhs_bounds = numeric_cast_precast_bounds (cx , normalized_rhs) ; upcast_comparison_bounds_err (cx , expr . span , rel , lhs_bounds , normalized_lhs , normalized_rhs , false) ; upcast_comparison_bounds_err (cx , expr . span , rel , rhs_bounds , normalized_rhs , normalized_lhs , true) ; } } }
};
}
