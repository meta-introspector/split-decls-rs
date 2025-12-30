// Generated macro for impl_11146 (impl)
macro_rules! Depcrate_unused_selfimpl_11146 {
() => {
// Module: crate::unused_self
// Provides: {"impl_11146"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for UnusedSelf { fn check_impl_item (& mut self , cx : & LateContext < 'tcx > , impl_item : & ImplItem < '_ >) { if impl_item . span . from_expansion () { return ; } let parent = cx . tcx . hir_get_parent_item (impl_item . hir_id ()) . def_id ; let parent_item = cx . tcx . hir_expect_item (parent) ; let assoc_item = cx . tcx . associated_item (impl_item . owner_id) ; if let ItemKind :: Impl (Impl { of_trait : None , .. }) = parent_item . kind && assoc_item . is_method () && let ImplItemKind :: Fn (.. , body_id) = & impl_item . kind && (! cx . effective_visibilities . is_exported (impl_item . owner_id . def_id) || ! self . avoid_breaking_exported_api) && let body = cx . tcx . hir_body (* body_id) && let [self_param , ..] = body . params && ! is_local_used (cx , body , self_param . pat . hir_id) && ! is_todo_unimplemented_stub (cx , body . value) { span_lint_and_help (cx , UNUSED_SELF , self_param . span , "unused `self` argument" , None , "consider refactoring to an associated function" ,) ; } } }
};
}
