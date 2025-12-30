// Generated macro for crate_inherent_impls (function)
macro_rules! Depcrate_coherence_inherent_implscrate_inherent_impls {
() => {
// Module: crate::coherence::inherent_impls
// Provides: {"crate_inherent_impls"}
// Dependencies: {}
# [doc = " On-demand query: yields a map containing all types mapped to their inherent impls."] pub (crate) fn crate_inherent_impls (tcx : TyCtxt < '_ > , () : () ,) -> (& '_ CrateInherentImpls , Result < () , ErrorGuaranteed >) { let mut collect = InherentCollect { tcx , impls_map : Default :: default () } ; let mut res = Ok (()) ; for id in tcx . hir_free_items () { res = res . and (collect . check_item (id)) ; } (tcx . arena . alloc (collect . impls_map) , res) }
};
}
