// Generated macro for impl_2904 (impl)
macro_rules! Depcrate_infallible_try_fromimpl_2904 {
() => {
// Module: crate::infallible_try_from
// Provides: {"impl_2904"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for InfallibleTryFrom { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < 'tcx >) { let ItemKind :: Impl (imp) = item . kind else { return } ; let Some (of_trait) = imp . of_trait else { return } ; let Some (trait_def_id) = of_trait . trait_ref . trait_def_id () else { return ; } ; if ! cx . tcx . is_diagnostic_item (sym :: TryFrom , trait_def_id) { return ; } for ii in cx . tcx . associated_items (item . owner_id . def_id) . filter_by_name_unhygienic_and_kind (sym :: Error , AssocTag :: Type) { let ii_ty = cx . tcx . type_of (ii . def_id) . instantiate_identity () ; if ! ii_ty . is_inhabited_from (cx . tcx , ii . def_id , cx . typing_env ()) { let mut span = MultiSpan :: from_span (cx . tcx . def_span (item . owner_id . to_def_id ())) ; let ii_ty_span = cx . tcx . hir_node_by_def_id (ii . def_id . expect_local ()) . expect_impl_item () . expect_type () . span ; span . push_span_label (ii_ty_span , "infallible error type") ; span_lint (cx , INFALLIBLE_TRY_FROM , span , "infallible TryFrom impl; consider implementing From instead" ,) ; } } } }
};
}
