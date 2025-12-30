// Generated macro for is_not_in_trait_impl (function)
macro_rules! Depcrate_min_ident_charsis_not_in_trait_impl {
() => {
// Module: crate::min_ident_chars
// Provides: {"is_not_in_trait_impl"}
// Dependencies: {}
# [doc = " Check if a pattern is a function param in an impl block for a trait and that the param name is"] # [doc = " the same than in the trait definition."] fn is_not_in_trait_impl (cx : & LateContext < '_ > , pat : & Pat < '_ > , ident : Ident) -> bool { let parent_node = cx . tcx . parent_hir_node (pat . hir_id) ; if ! matches ! (parent_node , Node :: Param (_)) { return true ; } for (_ , parent_node) in cx . tcx . hir_parent_iter (pat . hir_id) { if let Node :: ImplItem (impl_item) = parent_node && matches ! (impl_item . kind , ImplItemKind :: Fn (_ , _)) { let impl_parent_node = cx . tcx . parent_hir_node (impl_item . hir_id ()) ; if let Node :: Item (parent_item) = impl_parent_node && let ItemKind :: Impl (Impl { of_trait : Some (_) , .. }) = & parent_item . kind && let Some (name) = get_param_name (impl_item , cx , ident) { return name != ident . name ; } return true ; } } true }
};
}
