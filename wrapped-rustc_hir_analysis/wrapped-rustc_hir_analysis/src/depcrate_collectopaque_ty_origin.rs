// Generated macro for opaque_ty_origin (function)
macro_rules! Depcrate_collectopaque_ty_origin {
() => {
// Module: crate::collect
// Provides: {"opaque_ty_origin"}
// Dependencies: {}
fn opaque_ty_origin < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId) -> hir :: OpaqueTyOrigin < DefId > { match tcx . hir_node_by_def_id (def_id) . expect_opaque_ty () . origin { hir :: OpaqueTyOrigin :: FnReturn { parent , in_trait_or_impl } => { hir :: OpaqueTyOrigin :: FnReturn { parent : parent . to_def_id () , in_trait_or_impl } } hir :: OpaqueTyOrigin :: AsyncFn { parent , in_trait_or_impl } => { hir :: OpaqueTyOrigin :: AsyncFn { parent : parent . to_def_id () , in_trait_or_impl } } hir :: OpaqueTyOrigin :: TyAlias { parent , in_assoc_ty } => { hir :: OpaqueTyOrigin :: TyAlias { parent : parent . to_def_id () , in_assoc_ty } } } }
};
}
