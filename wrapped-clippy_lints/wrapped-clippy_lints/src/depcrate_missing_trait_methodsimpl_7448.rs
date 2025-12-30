// Generated macro for impl_7448 (impl)
macro_rules! Depcrate_missing_trait_methodsimpl_7448 {
() => {
// Module: crate::missing_trait_methods
// Provides: {"impl_7448"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for MissingTraitMethods { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < 'tcx >) { if ! is_lint_allowed (cx , MISSING_TRAIT_METHODS , item . hir_id ()) && span_is_local (item . span) && let ItemKind :: Impl (Impl { of_trait : Some (of_trait) , .. }) = item . kind && let Some (trait_id) = of_trait . trait_ref . trait_def_id () { let trait_item_ids : DefIdSet = cx . tcx . associated_items (item . owner_id) . in_definition_order () . filter_map (| assoc_item | assoc_item . expect_trait_impl () . ok ()) . collect () ; for assoc in cx . tcx . provided_trait_methods (trait_id) . filter (| assoc | ! trait_item_ids . contains (& assoc . def_id)) { span_lint_and_then (cx , MISSING_TRAIT_METHODS , cx . tcx . def_span (item . owner_id) , format ! ("missing trait method provided by default: `{}`" , assoc . name ()) , | diag | { diag . span_help (cx . tcx . def_span (assoc . def_id) , "implement the method") ; } ,) ; } } } }
};
}
