// Generated macro for impl_124 (impl)
macro_rules! Depcrate_msrv_attr_implimpl_124 {
() => {
// Module: crate::msrv_attr_impl
// Provides: {"impl_124"}
// Dependencies: {}
impl LateLintPass < '_ > for MsrvAttrImpl { fn check_item (& mut self , cx : & LateContext < '_ > , item : & hir :: Item < '_ >) { if let hir :: ItemKind :: Impl (hir :: Impl { of_trait : Some (_) , items , .. }) = & item . kind && let trait_ref = cx . tcx . impl_trait_ref (item . owner_id) . instantiate_identity () && internal_paths :: EARLY_LINT_PASS . matches (cx , trait_ref . def_id) && let ty :: Adt (self_ty_def , _) = trait_ref . self_ty () . kind () && self_ty_def . is_struct () && self_ty_def . all_fields () . any (| f | { cx . tcx . type_of (f . did) . instantiate_identity () . walk () . filter (| t | matches ! (t . kind () , GenericArgKind :: Type (_))) . any (| t | internal_paths :: MSRV_STACK . matches_ty (cx , t . expect_ty ())) }) && ! items . iter () . any (| & item | cx . tcx . item_name (item . owner_id) == sym :: check_attributes) { let span = cx . sess () . source_map () . span_through_char (item . span , '{') ; span_lint_and_sugg (cx , MISSING_MSRV_ATTR_IMPL , span , "`extract_msrv_attr!` macro missing from `EarlyLintPass` implementation" , "add `extract_msrv_attr!()` to the `EarlyLintPass` implementation" , format ! ("{}\n    extract_msrv_attr!();" , snippet (cx , span , "..")) , Applicability :: MachineApplicable ,) ; } } }
};
}
