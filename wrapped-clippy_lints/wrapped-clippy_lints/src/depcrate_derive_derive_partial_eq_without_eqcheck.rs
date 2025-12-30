// Generated macro for check (function)
macro_rules! Depcrate_derive_derive_partial_eq_without_eqcheck {
() => {
// Module: crate::derive::derive_partial_eq_without_eq
// Provides: {"check"}
// Dependencies: {}
# [doc = " Implementation of the `DERIVE_PARTIAL_EQ_WITHOUT_EQ` lint."] pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , span : Span , trait_ref : & hir :: TraitRef < '_ > , ty : Ty < 'tcx > , adt_hir_id : HirId ,) { if let ty :: Adt (adt , args) = ty . kind () && cx . tcx . visibility (adt . did ()) . is_public () && let Some (eq_trait_def_id) = cx . tcx . get_diagnostic_item (sym :: Eq) && let Some (def_id) = trait_ref . trait_def_id () && cx . tcx . is_diagnostic_item (sym :: PartialEq , def_id) && ! has_non_exhaustive_attr (cx . tcx , * adt) && ! ty_implements_eq_trait (cx . tcx , ty , eq_trait_def_id) && let typing_env = typing_env_for_derived_eq (cx . tcx , adt . did () , eq_trait_def_id) && adt . all_fields () . map (| f | f . ty (cx . tcx , args)) . all (| ty | implements_trait_with_env (cx . tcx , typing_env , ty , eq_trait_def_id , None , & [])) { span_lint_hir_and_then (cx , DERIVE_PARTIAL_EQ_WITHOUT_EQ , adt_hir_id , span . ctxt () . outer_expn_data () . call_site , "you are deriving `PartialEq` and can implement `Eq`" , | diag | { diag . span_suggestion (span . ctxt () . outer_expn_data () . call_site , "consider deriving `Eq` as well" , "PartialEq, Eq" , Applicability :: MachineApplicable ,) ; } ,) ; } }
};
}
