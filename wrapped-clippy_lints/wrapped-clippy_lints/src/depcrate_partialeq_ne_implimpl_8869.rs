// Generated macro for impl_8869 (impl)
macro_rules! Depcrate_partialeq_ne_implimpl_8869 {
() => {
// Module: crate::partialeq_ne_impl
// Provides: {"impl_8869"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for PartialEqNeImpl { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < '_ >) { if let ItemKind :: Impl (Impl { of_trait : Some (of_trait) , items : impl_items , .. }) = item . kind && ! cx . tcx . is_automatically_derived (item . owner_id . to_def_id ()) && let Some (eq_trait) = cx . tcx . lang_items () . eq_trait () && of_trait . trait_ref . path . res . def_id () == eq_trait { for impl_item in impl_items { if cx . tcx . item_name (impl_item . owner_id) == sym :: ne { span_lint_hir (cx , PARTIALEQ_NE_IMPL , impl_item . hir_id () , cx . tcx . def_span (impl_item . owner_id) , "re-implementing `PartialEq::ne` is unnecessary" ,) ; } } } } }
};
}
