// Generated macro for check (function)
macro_rules! Depcrate_derive_derive_ord_xor_partial_ordcheck {
() => {
// Module: crate::derive::derive_ord_xor_partial_ord
// Provides: {"check"}
// Dependencies: {}
# [doc = " Implementation of the `DERIVE_ORD_XOR_PARTIAL_ORD` lint."] pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , span : Span , trait_ref : & hir :: TraitRef < '_ > , ty : Ty < 'tcx > , adt_hir_id : HirId , ord_is_automatically_derived : bool ,) { if let Some (ord_trait_def_id) = cx . tcx . get_diagnostic_item (sym :: Ord) && let Some (partial_ord_trait_def_id) = cx . tcx . lang_items () . partial_ord_trait () && let Some (def_id) = & trait_ref . trait_def_id () && * def_id == ord_trait_def_id { cx . tcx . for_each_relevant_impl (partial_ord_trait_def_id , ty , | impl_id | { let partial_ord_is_automatically_derived = cx . tcx . is_automatically_derived (impl_id) ; if partial_ord_is_automatically_derived == ord_is_automatically_derived { return ; } let trait_ref = cx . tcx . impl_trait_ref (impl_id) ; if trait_ref . instantiate_identity () . args . type_at (1) == ty { let mess = if partial_ord_is_automatically_derived { "you are implementing `Ord` explicitly but have derived `PartialOrd`" } else { "you are deriving `Ord` but have implemented `PartialOrd` explicitly" } ; span_lint_hir_and_then (cx , DERIVE_ORD_XOR_PARTIAL_ORD , adt_hir_id , span , mess , | diag | { if let Some (local_def_id) = impl_id . as_local () { let hir_id = cx . tcx . local_def_id_to_hir_id (local_def_id) ; diag . span_note (cx . tcx . hir_span (hir_id) , "`PartialOrd` implemented here") ; } }) ; } }) ; } }
};
}
