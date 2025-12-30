// Generated macro for impl_8610 (impl)
macro_rules! Depcrate_panic_in_result_fnimpl_8610 {
() => {
// Module: crate::panic_in_result_fn
// Provides: {"impl_8610"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for PanicInResultFn { fn check_fn (& mut self , cx : & LateContext < 'tcx > , fn_kind : FnKind < 'tcx > , _ : & 'tcx hir :: FnDecl < 'tcx > , body : & 'tcx hir :: Body < 'tcx > , span : Span , def_id : LocalDefId ,) { if matches ! (fn_kind , FnKind :: Closure) { return ; } let owner = cx . tcx . local_def_id_to_hir_id (def_id) . expect_owner () ; if is_type_diagnostic_item (cx , return_ty (cx , owner) , sym :: Result) { lint_impl_body (cx , span , body) ; } } }
};
}
