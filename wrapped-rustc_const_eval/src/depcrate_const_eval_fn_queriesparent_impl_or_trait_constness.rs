// Generated macro for parent_impl_or_trait_constness (function)
macro_rules! Depcrate_const_eval_fn_queriesparent_impl_or_trait_constness {
() => {
// Module: crate::const_eval::fn_queries
// Provides: {"parent_impl_or_trait_constness"}
// Dependencies: {}
fn parent_impl_or_trait_constness (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> hir :: Constness { let parent_id = tcx . local_parent (def_id) ; match tcx . def_kind (parent_id) { DefKind :: Impl { of_trait : true } => tcx . impl_trait_header (parent_id) . unwrap () . constness , DefKind :: Trait => { if tcx . is_const_trait (parent_id . into ()) { hir :: Constness :: Const } else { hir :: Constness :: NotConst } } _ => hir :: Constness :: NotConst , } }
};
}
