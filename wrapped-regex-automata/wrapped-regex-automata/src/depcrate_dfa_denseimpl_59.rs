// Generated macro for impl_59 (impl)
macro_rules! Depcrate_dfa_denseimpl_59 {
() => {
// Module: crate::dfa::dense
// Provides: {"impl_59"}
// Dependencies: {}
impl < 'a , T : AsRef < [u32] > > Iterator for StateIter < 'a , T > { type Item = State < 'a > ; fn next (& mut self) -> Option < State < 'a > > { self . it . next () . map (| (index , _) | { let id = self . tt . to_state_id (index) ; self . tt . state (id) }) } }
};
}
