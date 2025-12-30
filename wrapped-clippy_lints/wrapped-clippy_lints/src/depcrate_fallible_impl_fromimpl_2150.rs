// Generated macro for impl_2150 (impl)
macro_rules! Depcrate_fallible_impl_fromimpl_2150 {
() => {
// Module: crate::fallible_impl_from
// Provides: {"impl_2150"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for FallibleImplFrom { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx hir :: Item < '_ >) { if let hir :: ItemKind :: Impl (_) = & item . kind && let Some (impl_trait_ref) = cx . tcx . impl_trait_ref (item . owner_id) && cx . tcx . is_diagnostic_item (sym :: From , impl_trait_ref . skip_binder () . def_id) { lint_impl_body (cx , item . owner_id , item . span) ; } } }
};
}
