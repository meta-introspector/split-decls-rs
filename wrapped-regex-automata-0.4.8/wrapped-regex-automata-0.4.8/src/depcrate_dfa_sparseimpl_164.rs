// Generated macro for impl_164 (impl)
macro_rules! Depcrate_dfa_sparseimpl_164 {
() => {
// Module: crate::dfa::sparse
// Provides: {"impl_164"}
// Dependencies: {}
# [cfg (feature = "dfa-build")] impl < 'a > StateMut < 'a > { # [doc = " Sets the ith transition to the given state."] fn set_next_at (& mut self , i : usize , next : StateID) { let start = i * StateID :: SIZE ; let end = start + StateID :: SIZE ; wire :: write_state_id :: < wire :: NE > (next , & mut self . next [start .. end]) ; } }
};
}
