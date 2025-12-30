// Generated macro for check_trait_item (function)
macro_rules! Depcrate_functions_impl_trait_in_paramscheck_trait_item {
() => {
// Module: crate::functions::impl_trait_in_params
// Provides: {"check_trait_item"}
// Dependencies: {}
pub (super) fn check_trait_item (cx : & LateContext < '_ > , trait_item : & TraitItem < '_ > , avoid_breaking_exported_api : bool) { if ! avoid_breaking_exported_api && let TraitItemKind :: Fn (_ , _) = trait_item . kind && let hir :: Node :: Item (item) = cx . tcx . parent_hir_node (trait_item . hir_id ()) && ! item . vis_span . is_empty () && ! is_in_test (cx . tcx , trait_item . hir_id ()) { for param in trait_item . generics . params { if param . is_impl_trait () { report (cx , param , trait_item . generics) ; } } } }
};
}
