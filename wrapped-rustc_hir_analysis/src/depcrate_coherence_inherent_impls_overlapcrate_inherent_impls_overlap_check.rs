// Generated macro for crate_inherent_impls_overlap_check (function)
macro_rules! Depcrate_coherence_inherent_impls_overlapcrate_inherent_impls_overlap_check {
() => {
// Module: crate::coherence::inherent_impls_overlap
// Provides: {"crate_inherent_impls_overlap_check"}
// Dependencies: {}
pub (crate) fn crate_inherent_impls_overlap_check (tcx : TyCtxt < '_ > , () : () ,) -> Result < () , ErrorGuaranteed > { let mut inherent_overlap_checker = InherentOverlapChecker { tcx } ; let mut res = Ok (()) ; for id in tcx . hir_free_items () { res = res . and (inherent_overlap_checker . check_item (id)) ; } res }
};
}
