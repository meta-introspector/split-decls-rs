// Generated macro for check_impl_item (function)
macro_rules! Depcrate_functions_impl_trait_in_paramscheck_impl_item {
() => {
// Module: crate::functions::impl_trait_in_params
// Provides: {"check_impl_item"}
// Dependencies: {}
pub (super) fn check_impl_item (cx : & LateContext < '_ > , impl_item : & ImplItem < '_ >) { if let ImplItemKind :: Fn (_ , body_id) = impl_item . kind && let hir :: Node :: Item (item) = cx . tcx . parent_hir_node (impl_item . hir_id ()) && let hir :: ItemKind :: Impl (impl_) = item . kind && let hir :: Impl { of_trait : None , .. } = impl_ && let body = cx . tcx . hir_body (body_id) && cx . tcx . visibility (cx . tcx . hir_body_owner_def_id (body . id ())) . is_public () && ! is_in_test (cx . tcx , impl_item . hir_id ()) { for param in impl_item . generics . params { if param . is_impl_trait () { report (cx , param , impl_item . generics) ; } } } }
};
}
