// Generated macro for inherent_impls (function)
macro_rules! Depcrate_coherence_inherent_implsinherent_impls {
() => {
// Module: crate::coherence::inherent_impls
// Provides: {"inherent_impls"}
// Dependencies: {}
# [doc = " On-demand query: yields a vector of the inherent impls for a specific type."] pub (crate) fn inherent_impls (tcx : TyCtxt < '_ > , ty_def_id : LocalDefId) -> & [DefId] { let (crate_map , _) = tcx . crate_inherent_impls (()) ; match crate_map . inherent_impls . get (& ty_def_id) { Some (v) => & v [..] , None => & [] , } }
};
}
