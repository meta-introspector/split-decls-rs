// Generated macro for impl_10857 (impl)
macro_rules! Depcrate_unnecessary_box_returnsimpl_10857 {
() => {
// Module: crate::unnecessary_box_returns
// Provides: {"impl_10857"}
// Dependencies: {}
impl UnnecessaryBoxReturns { pub fn new (conf : & 'static Conf) -> Self { Self { avoid_breaking_exported_api : conf . avoid_breaking_exported_api , maximum_size : conf . unnecessary_box_size , } } fn check_fn_item (& self , cx : & LateContext < '_ > , decl : & FnDecl < '_ > , def_id : LocalDefId , name : Symbol) { if self . avoid_breaking_exported_api && cx . effective_visibilities . is_exported (def_id) { return ; } if name . as_str () . contains ("box") { return ; } let FnRetTy :: Return (return_ty_hir) = & decl . output else { return ; } ; let return_ty = cx . tcx . instantiate_bound_regions_with_erased (cx . tcx . fn_sig (def_id) . skip_binder ()) . output () ; let Some (boxed_ty) = return_ty . boxed_ty () else { return ; } ; if boxed_ty . is_sized (cx . tcx , cx . typing_env ()) && approx_ty_size (cx , boxed_ty) <= self . maximum_size { span_lint_and_then (cx , UNNECESSARY_BOX_RETURNS , return_ty_hir . span , format ! ("boxed return of the sized type `{boxed_ty}`") , | diagnostic | { diagnostic . span_suggestion (return_ty_hir . span , "try" , boxed_ty . to_string () , Applicability :: Unspecified ,) ; diagnostic . help ("changing this also requires a change to the return expressions in this function") ; } ,) ; } } }
};
}
