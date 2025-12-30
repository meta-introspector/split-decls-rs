// Generated macro for impl_486 (impl)
macro_rules! Depcrate_util_remapperimpl_486 {
() => {
// Module: crate::util::remapper
// Provides: {"impl_486"}
// Dependencies: {}
impl IndexMapper { # [doc = " Convert a state ID to a state index."] fn to_index (& self , id : StateID) -> usize { id . as_usize () >> self . stride2 } # [doc = " Convert a state index to a state ID."] fn to_state_id (& self , index : usize) -> StateID { StateID :: new_unchecked (index << self . stride2) } }
};
}
