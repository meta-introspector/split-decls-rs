// Generated macro for impl_161 (impl)
macro_rules! Depcrate_dfa_sparseimpl_161 {
() => {
// Module: crate::dfa::sparse
// Provides: {"impl_161"}
// Dependencies: {}
impl < 'a , T : AsRef < [u8] > > Iterator for StateIter < 'a , T > { type Item = State < 'a > ; fn next (& mut self) -> Option < State < 'a > > { if self . id >= self . trans . sparse () . len () { return None ; } let state = self . trans . state (StateID :: new_unchecked (self . id)) ; self . id = self . id + state . write_to_len () ; Some (state) } }
};
}
