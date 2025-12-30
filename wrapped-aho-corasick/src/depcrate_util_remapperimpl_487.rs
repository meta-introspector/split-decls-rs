// Generated macro for impl_487 (impl)
macro_rules! Depcrate_util_remapperimpl_487 {
() => {
// Module: crate::util::remapper
// Provides: {"impl_487"}
// Dependencies: {}
impl Remappable for noncontiguous :: NFA { fn state_len (& self) -> usize { noncontiguous :: NFA :: states (self) . len () } fn swap_states (& mut self , id1 : StateID , id2 : StateID) { noncontiguous :: NFA :: swap_states (self , id1 , id2) } fn remap (& mut self , map : impl Fn (StateID) -> StateID) { noncontiguous :: NFA :: remap (self , map) } }
};
}
