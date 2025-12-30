// Generated macro for impl_7301 (impl)
macro_rules! Depcrate_minmaximpl_7301 {
() => {
// Module: crate::minmax
// Provides: {"impl_7301"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for MinMaxPass { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if let Some ((outer_max , outer_c , oe)) = min_max (cx , expr) && let Some ((inner_max , inner_c , ie)) = min_max (cx , oe) && outer_max != inner_max && let Some (ord) = Constant :: partial_cmp (cx . tcx , cx . typeck_results () . expr_ty (ie) , & outer_c , & inner_c) && matches ! ((outer_max , ord) , (MinMax :: Max , Equal | Greater) | (MinMax :: Min , Equal | Less)) { span_lint (cx , MIN_MAX , expr . span , "this `min`/`max` combination leads to constant result" ,) ; } } }
};
}
