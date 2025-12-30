// Generated macro for impl_2725 (impl)
macro_rules! Depcrate_implicit_returnimpl_2725 {
() => {
// Module: crate::implicit_return
// Provides: {"impl_2725"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for ImplicitReturn { fn check_fn (& mut self , cx : & LateContext < 'tcx > , kind : FnKind < 'tcx > , decl : & 'tcx FnDecl < '_ > , body : & 'tcx Body < '_ > , span : Span , _ : LocalDefId ,) { if (! matches ! (kind , FnKind :: Closure) && matches ! (decl . output , FnRetTy :: DefaultReturn (_))) || ! span . eq_ctxt (body . value . span) || span . in_external_macro (cx . sess () . source_map ()) { return ; } let res_ty = cx . typeck_results () . expr_ty (body . value) ; if res_ty . is_unit () || res_ty . is_never () { return ; } let expr = if is_async_fn (kind) { match get_async_fn_body (cx . tcx , body) { Some (e) => e , None => return , } } else if let Some (expr) = get_async_closure_expr (cx . tcx , body . value) { expr } else { body . value } ; if is_from_proc_macro (cx , expr) { return ; } lint_implicit_returns (cx , expr , expr . span . ctxt () , None) ; } }
};
}
