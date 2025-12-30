// Generated macro for impl_1291 (impl)
macro_rules! Depcrate_copy_iteratorimpl_1291 {
() => {
// Module: crate::copy_iterator
// Provides: {"impl_1291"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for CopyIterator { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < '_ >) { if let ItemKind :: Impl (Impl { of_trait : Some (of_trait) , .. }) = item . kind && let ty = cx . tcx . type_of (item . owner_id) . instantiate_identity () && is_copy (cx , ty) && let Some (trait_id) = of_trait . trait_ref . trait_def_id () && cx . tcx . is_diagnostic_item (sym :: Iterator , trait_id) { span_lint_and_note (cx , COPY_ITERATOR , item . span , "you are implementing `Iterator` on a `Copy` type" , None , "consider implementing `IntoIterator` instead" ,) ; } } }
};
}
