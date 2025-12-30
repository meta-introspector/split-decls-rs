// Generated macro for get_impl_span (function)
macro_rules! Depcrate_inherent_implget_impl_span {
() => {
// Module: crate::inherent_impl
// Provides: {"get_impl_span"}
// Dependencies: {}
# [doc = " Gets the span for the given impl block unless it's not being considered by the lint."] fn get_impl_span (cx : & LateContext < '_ > , id : LocalDefId) -> Option < Span > { let id = cx . tcx . local_def_id_to_hir_id (id) ; if let Node :: Item (& Item { kind : ItemKind :: Impl (impl_item) , span , .. }) = cx . tcx . hir_node (id) { (! span . from_expansion () && impl_item . generics . params . is_empty () && ! fulfill_or_allowed (cx , MULTIPLE_INHERENT_IMPL , [id])) . then_some (span) } else { None } }
};
}
