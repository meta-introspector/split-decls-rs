// Generated macro for onepass (module)
macro_rules! Depcrate_dfa_remapperonepass {
() => {
// Module: crate::dfa::remapper
// Provides: {"onepass"}
// Dependencies: {}
# [cfg (feature = "dfa-onepass")] mod onepass { use crate :: { dfa :: onepass :: DFA , util :: primitives :: StateID } ; use super :: Remappable ; impl Remappable for DFA { fn state_len (& self) -> usize { DFA :: state_len (self) } fn stride2 (& self) -> usize { 0 } fn swap_states (& mut self , id1 : StateID , id2 : StateID) { DFA :: swap_states (self , id1 , id2) } fn remap (& mut self , map : impl Fn (StateID) -> StateID) { DFA :: remap (self , map) } } }
};
}
