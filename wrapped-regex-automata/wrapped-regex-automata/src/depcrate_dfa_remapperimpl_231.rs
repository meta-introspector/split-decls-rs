// Generated macro for impl_231 (impl)
macro_rules! Depcrate_dfa_remapperimpl_231 {
() => {
// Module: crate::dfa::remapper
// Provides: {"impl_231"}
// Dependencies: {}
impl IndexMapper { # [doc = " Convert a state ID to a state index."] fn to_index (& self , id : StateID) -> usize { id . as_usize () >> self . stride2 } # [doc = " Convert a state index to a state ID."] fn to_state_id (& self , index : usize) -> StateID { StateID :: new_unchecked (index << self . stride2) } }
};
}
