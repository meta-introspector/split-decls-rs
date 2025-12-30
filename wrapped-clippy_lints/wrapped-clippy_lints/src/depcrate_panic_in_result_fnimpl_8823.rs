// Generated macro for impl_8823 (impl)
macro_rules! Depcrate_panic_in_result_fnimpl_8823 {
() => {
// Module: crate::panic_in_result_fn
// Provides: {"impl_8823"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for PanicInResultFn { fn check_fn (& mut self , cx : & LateContext < 'tcx > , fn_kind : FnKind < 'tcx > , _ : & 'tcx hir :: FnDecl < 'tcx > , body : & 'tcx hir :: Body < 'tcx > , span : Span , def_id : LocalDefId ,) { if matches ! (fn_kind , FnKind :: Closure) { return ; } let owner = cx . tcx . local_def_id_to_hir_id (def_id) . expect_owner () ; if return_ty (cx , owner) . is_diag_item (cx , sym :: Result) { lint_impl_body (cx , span , body) ; } } }
};
}
