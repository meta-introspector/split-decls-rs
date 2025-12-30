// Generated macro for coherent_trait (function)
macro_rules! Depcrate_coherencecoherent_trait {
() => {
// Module: crate::coherence
// Provides: {"coherent_trait"}
// Dependencies: {}
fn coherent_trait (tcx : TyCtxt < '_ > , def_id : DefId) -> Result < () , ErrorGuaranteed > { let impls = tcx . local_trait_impls (def_id) ; if impls . is_empty () { return Ok (()) ; } let mut res = tcx . ensure_ok () . specialization_graph_of (def_id) ; for & impl_def_id in impls { let impl_header = tcx . impl_trait_header (impl_def_id) . unwrap () ; let trait_ref = impl_header . trait_ref . instantiate_identity () ; let trait_def = tcx . trait_def (trait_ref . def_id) ; res = res . and (check_impl (tcx , impl_def_id , trait_ref , trait_def , impl_header . polarity)) . and (check_object_overlap (tcx , impl_def_id , trait_ref)) . and (unsafety :: check_item (tcx , impl_def_id , impl_header , trait_def)) . and (tcx . ensure_ok () . orphan_check_impl (impl_def_id)) . and (builtin :: check_trait (tcx , def_id , impl_def_id , impl_header)) ; } res }
};
}
