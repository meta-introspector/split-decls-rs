// Generated macro for dense (module)
macro_rules! Depcrate_dfa_remapperdense {
() => {
// Module: crate::dfa::remapper
// Provides: {"dense"}
// Dependencies: {}
# [cfg (feature = "dfa-build")] mod dense { use crate :: { dfa :: dense :: OwnedDFA , util :: primitives :: StateID } ; use super :: Remappable ; impl Remappable for OwnedDFA { fn state_len (& self) -> usize { OwnedDFA :: state_len (self) } fn stride2 (& self) -> usize { OwnedDFA :: stride2 (self) } fn swap_states (& mut self , id1 : StateID , id2 : StateID) { OwnedDFA :: swap_states (self , id1 , id2) } fn remap (& mut self , map : impl Fn (StateID) -> StateID) { OwnedDFA :: remap (self , map) } } }
};
}
