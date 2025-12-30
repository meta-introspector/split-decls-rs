// Generated macro for get_impl_trait_def_id (function)
macro_rules! Depcrate_unconditional_recursionget_impl_trait_def_id {
() => {
// Module: crate::unconditional_recursion
// Provides: {"get_impl_trait_def_id"}
// Dependencies: {}
fn get_impl_trait_def_id (cx : & LateContext < '_ > , method_def_id : LocalDefId) -> Option < DefId > { let hir_id = cx . tcx . local_def_id_to_hir_id (method_def_id) ; if let Some ((_ , Node :: Item (Item { kind : ItemKind :: Impl (impl_) , owner_id , .. }) ,)) = cx . tcx . hir_parent_iter (hir_id) . next () && ! cx . tcx . is_automatically_derived (owner_id . to_def_id ()) && let Some (of_trait) = impl_ . of_trait { of_trait . trait_ref . trait_def_id () } else { None } }
};
}
