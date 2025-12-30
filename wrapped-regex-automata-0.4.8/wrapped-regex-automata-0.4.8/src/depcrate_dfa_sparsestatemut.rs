// Generated macro for StateMut (struct)
macro_rules! Depcrate_dfa_sparseStateMut {
() => {
// Module: crate::dfa::sparse
// Provides: {"StateMut"}
// Dependencies: {}
# [doc = " A representation of a mutable sparse DFA state that can be cheaply"] # [doc = " materialized from a state identifier."] # [cfg (feature = "dfa-build")] struct StateMut < 'a > { # [doc = " The identifier of this state."] id : StateID , # [doc = " Whether this is a match state or not."] is_match : bool , # [doc = " The number of transitions in this state."] ntrans : usize , # [doc = " Pairs of input ranges, where there is one pair for each transition."] # [doc = " Each pair specifies an inclusive start and end byte range for the"] # [doc = " corresponding transition."] input_ranges : & 'a mut [u8] , # [doc = " Transitions to the next state. This slice contains native endian"] # [doc = " encoded state identifiers, with `S` as the representation. Thus, there"] # [doc = " are `ntrans * size_of::<S>()` bytes in this slice."] next : & 'a mut [u8] , # [doc = " If this is a match state, then this contains the pattern IDs that match"] # [doc = " when the DFA is in this state."] # [doc = ""] # [doc = " This is a contiguous sequence of 32-bit native endian encoded integers."] pattern_ids : & 'a [u8] , # [doc = " An accelerator for this state, if present. If this state has no"] # [doc = " accelerator, then this is an empty slice. When non-empty, this slice"] # [doc = " has length at most 3 and corresponds to the exhaustive set of bytes"] # [doc = " that must be seen in order to transition out of this state."] accel : & 'a mut [u8] , }
};
}
