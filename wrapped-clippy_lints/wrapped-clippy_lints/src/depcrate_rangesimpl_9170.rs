// Generated macro for impl_9170 (impl)
macro_rules! Depcrate_rangesimpl_9170 {
() => {
// Module: crate::ranges
// Provides: {"impl_9170"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for Ranges { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if let ExprKind :: Binary (ref op , l , r) = expr . kind && self . msrv . meets (cx , msrvs :: RANGE_CONTAINS) { check_possible_range_contains (cx , op . node , l , r , expr , expr . span) ; } check_exclusive_range_plus_one (cx , expr) ; check_inclusive_range_minus_one (cx , expr) ; check_reversed_empty_range (cx , expr) ; } }
};
}
