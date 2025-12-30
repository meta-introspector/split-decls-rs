// Generated macro for impl_2037 (impl)
macro_rules! Depcrate_escapeimpl_2037 {
() => {
// Module: crate::escape
// Provides: {"impl_2037"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for BoxedLocal { fn check_fn (& mut self , cx : & LateContext < 'tcx > , fn_kind : intravisit :: FnKind < 'tcx > , _ : & 'tcx FnDecl < '_ > , body : & 'tcx Body < '_ > , _ : Span , fn_def_id : LocalDefId ,) { if let Some (header) = fn_kind . header () && header . abi != ExternAbi :: Rust { return ; } let parent_id = cx . tcx . hir_get_parent_item (cx . tcx . local_def_id_to_hir_id (fn_def_id)) . def_id ; let mut trait_self_ty = None ; match cx . tcx . def_kind (parent_id) { DefKind :: Impl { of_trait : true } => return , DefKind :: Trait => { trait_self_ty = Some (TraitRef :: identity (cx . tcx , parent_id . to_def_id ()) . self_ty ()) ; } , _ => { } , } let mut v = EscapeDelegate { cx , set : HirIdSet :: default () , trait_self_ty , too_large_for_stack : self . too_large_for_stack , } ; ExprUseVisitor :: for_clippy (cx , fn_def_id , & mut v) . consume_body (body) . into_ok () ; for node in v . set { span_lint_hir (cx , BOXED_LOCAL , node , cx . tcx . hir_span (node) , "local variable doesn't need to be boxed here" ,) ; } } }
};
}
