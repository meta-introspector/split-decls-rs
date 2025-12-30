// Generated macro for impl_1151 (impl)
macro_rules! Depcrate_cloned_ref_to_slice_refsimpl_1151 {
() => {
// Module: crate::cloned_ref_to_slice_refs
// Provides: {"impl_1151"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for ClonedRefToSliceRefs < '_ > { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & Expr < 'tcx >) { if self . msrv . meets (cx , { if is_in_const_context (cx) { msrvs :: CONST_SLICE_FROM_REF } else { msrvs :: SLICE_FROM_REF } }) && let ExprKind :: AddrOf (_ , mutability , arr) = & expr . kind && mutability . is_not () && let ExprKind :: Array ([item]) = & arr . kind && let ExprKind :: MethodCall (_ , val , _ , _) = item . kind && is_trait_method (cx , item , sym :: Clone) && (! is_mutable (cx , val) || is_const_evaluatable (cx , val)) && let Some (builtin_crate) = clippy_utils :: std_or_core (cx) { let mut sugg = Sugg :: hir (cx , val , "_") ; if ! cx . typeck_results () . expr_ty (val) . is_ref () { sugg = sugg . addr () ; } span_lint_and_sugg (cx , CLONED_REF_TO_SLICE_REFS , expr . span , format ! ("this call to `clone` can be replaced with `{builtin_crate}::slice::from_ref`") , "try" , format ! ("{builtin_crate}::slice::from_ref({sugg})") , Applicability :: MaybeIncorrect ,) ; } } }
};
}
