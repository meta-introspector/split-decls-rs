// Generated macro for impl_378 (impl)
macro_rules! Depcrate_autoderefimpl_378 {
() => {
// Module: crate::autoderef
// Provides: {"impl_378"}
// Dependencies: {}
impl < T : TrackAutoderefSteps > Iterator for Autoderef < '_ , '_ , T > { type Item = (Ty , usize) ; # [tracing :: instrument (skip_all)] fn next (& mut self) -> Option < Self :: Item > { if mem :: take (& mut self . at_start) { return Some ((self . ty . clone () , 0)) ; } if self . steps . len () > AUTODEREF_RECURSION_LIMIT { return None ; } let (kind , new_ty) = autoderef_step (self . table , self . ty . clone () , self . explicit , self . use_receiver_trait) ? ; self . steps . push (kind , & self . ty) ; self . ty = new_ty ; Some ((self . ty . clone () , self . step_count ())) } }
};
}
