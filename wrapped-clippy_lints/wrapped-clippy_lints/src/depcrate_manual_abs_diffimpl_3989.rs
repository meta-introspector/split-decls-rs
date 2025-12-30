// Generated macro for impl_3989 (impl)
macro_rules! Depcrate_manual_abs_diffimpl_3989 {
() => {
// Module: crate::manual_abs_diff
// Provides: {"impl_3989"}
// Dependencies: {}
impl ManualAbsDiff { # [doc = " Returns a type if `a` and `b` are both of it, and this lint can be applied to that"] # [doc = " type (currently, any primitive int, or a `Duration`)"] fn are_ty_eligible < 'tcx > (& self , cx : & LateContext < 'tcx > , a : & Expr < '_ > , b : & Expr < '_ >) -> Option < (Ty < 'tcx > , usize) > { let is_int = | ty : Ty < '_ > | matches ! (ty . kind () , ty :: Uint (_) | ty :: Int (_)) && self . msrv . meets (cx , msrvs :: ABS_DIFF) ; let is_duration = | ty | is_type_diagnostic_item (cx , ty , sym :: Duration) && self . msrv . meets (cx , msrvs :: DURATION_ABS_DIFF) ; let a_ty = cx . typeck_results () . expr_ty (a) . peel_refs () ; let (b_ty , b_n_refs) = peel_middle_ty_refs (cx . typeck_results () . expr_ty (b)) ; (a_ty == b_ty && (is_int (a_ty) || is_duration (a_ty))) . then_some ((a_ty , b_n_refs)) } }
};
}
