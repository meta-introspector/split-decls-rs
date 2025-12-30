// Generated macro for impl_20 (impl)
macro_rules! Depcrate_async_fn_in_traitimpl_20 {
() => {
// Module: crate::async_fn_in_trait
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for AsyncFnInTrait { fn check_trait_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx hir :: TraitItem < 'tcx >) { if let hir :: TraitItemKind :: Fn (sig , body) = item . kind && let hir :: IsAsync :: Async (async_span) = sig . header . asyncness { if cx . tcx . features () . return_type_notation () { return ; } if ! cx . tcx . effective_visibilities (()) . is_reachable (item . owner_id . def_id) { return ; } let hir :: FnRetTy :: Return (hir :: Ty { kind : hir :: TyKind :: OpaqueDef (opaq_def , ..) , .. }) = sig . decl . output else { return ; } ; let sugg = suggest_desugaring_async_fn_to_impl_future_in_trait (cx . tcx , sig , body , opaq_def . def_id , " + Send" ,) ; cx . tcx . emit_node_span_lint (ASYNC_FN_IN_TRAIT , item . hir_id () , async_span , AsyncFnInTraitDiag { sugg } ,) ; } } }
};
}
