// Generated macro for check_partial_eq_without_eq (function)
macro_rules! Depcrate_derivecheck_partial_eq_without_eq {
() => {
// Module: crate::derive
// Provides: {"check_partial_eq_without_eq"}
// Dependencies: {}
# [doc = " Implementation of the `DERIVE_PARTIAL_EQ_WITHOUT_EQ` lint."] fn check_partial_eq_without_eq < 'tcx > (cx : & LateContext < 'tcx > , span : Span , trait_ref : & hir :: TraitRef < '_ > , ty : Ty < 'tcx >) { if let ty :: Adt (adt , args) = ty . kind () && cx . tcx . visibility (adt . did ()) . is_public () && let Some (eq_trait_def_id) = cx . tcx . get_diagnostic_item (sym :: Eq) && let Some (def_id) = trait_ref . trait_def_id () && cx . tcx . is_diagnostic_item (sym :: PartialEq , def_id) && ! has_non_exhaustive_attr (cx . tcx , * adt) && ! ty_implements_eq_trait (cx . tcx , ty , eq_trait_def_id) && let typing_env = typing_env_for_derived_eq (cx . tcx , adt . did () , eq_trait_def_id) && let Some (local_def_id) = adt . did () . as_local () && adt . all_fields () . map (| f | f . ty (cx . tcx , args)) . all (| ty | implements_trait_with_env (cx . tcx , typing_env , ty , eq_trait_def_id , None , & [])) { span_lint_hir_and_then (cx , DERIVE_PARTIAL_EQ_WITHOUT_EQ , cx . tcx . local_def_id_to_hir_id (local_def_id) , span . ctxt () . outer_expn_data () . call_site , "you are deriving `PartialEq` and can implement `Eq`" , | diag | { diag . span_suggestion (span . ctxt () . outer_expn_data () . call_site , "consider deriving `Eq` as well" , "PartialEq, Eq" , Applicability :: MachineApplicable ,) ; } ,) ; } }
};
}
