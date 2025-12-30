// Generated macro for is_default_trait_impl (function)
macro_rules! Depcrate_unused_asyncis_default_trait_impl {
() => {
// Module: crate::unused_async
// Provides: {"is_default_trait_impl"}
// Dependencies: {}
fn is_default_trait_impl (cx : & LateContext < '_ > , def_id : LocalDefId) -> bool { matches ! (cx . tcx . hir_node_by_def_id (def_id) , Node :: TraitItem (TraitItem { defaultness : Defaultness :: Default { .. } , .. })) }
};
}
