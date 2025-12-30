// Generated macro for impl_2179 (impl)
macro_rules! Depcrate_fallible_impl_fromimpl_2179 {
() => {
// Module: crate::fallible_impl_from
// Provides: {"impl_2179"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for FallibleImplFrom { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx hir :: Item < '_ >) { if let hir :: ItemKind :: Impl (hir :: Impl { of_trait : Some (_) , .. }) = & item . kind && let impl_trait_id = cx . tcx . impl_trait_id (item . owner_id) && cx . tcx . is_diagnostic_item (sym :: From , impl_trait_id) { lint_impl_body (cx , item . owner_id , item . span) ; } } }
};
}
