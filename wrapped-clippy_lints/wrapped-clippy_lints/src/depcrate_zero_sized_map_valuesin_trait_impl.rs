// Generated macro for in_trait_impl (function)
macro_rules! Depcrate_zero_sized_map_valuesin_trait_impl {
() => {
// Module: crate::zero_sized_map_values
// Provides: {"in_trait_impl"}
// Dependencies: {}
fn in_trait_impl (cx : & LateContext < '_ > , hir_id : HirId) -> bool { let parent_id = cx . tcx . hir_get_parent_item (hir_id) ; let second_parent_id = cx . tcx . hir_get_parent_item (parent_id . into ()) . def_id ; if let Node :: Item (item) = cx . tcx . hir_node_by_def_id (second_parent_id) && let ItemKind :: Impl (hir :: Impl { of_trait : Some (_) , .. }) = item . kind { return true ; } false }
};
}
