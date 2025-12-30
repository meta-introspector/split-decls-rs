// Generated macro for impl_748 (impl)
macro_rules! Depcrate_method_probeimpl_748 {
() => {
// Module: crate::method::probe
// Provides: {"impl_748"}
// Dependencies: {}
impl PickConstraintsForShadowed { fn may_shadow_based_on_autoderefs (& self , autoderefs : usize) -> bool { autoderefs == self . autoderefs } fn candidate_may_shadow (& self , candidate : & Candidate < '_ >) -> bool { candidate . item . def_id != self . def_id && match candidate . kind { CandidateKind :: InherentImplCandidate { receiver_steps , .. } => match self . receiver_steps { Some (shadowed_receiver_steps) => receiver_steps > shadowed_receiver_steps , _ => false } , _ => false } } }
};
}
