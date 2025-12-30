// Generated macro for State (struct)
macro_rules! Depcrate_dfa_sparseState {
() => {
// Module: crate::dfa::sparse
// Provides: {"State"}
// Dependencies: {}
# [doc = " A representation of a sparse DFA state that can be cheaply materialized"] # [doc = " from a state identifier."] # [derive (Clone)] struct State < 'a > { # [doc = " The identifier of this state."] id : StateID , # [doc = " Whether this is a match state or not."] is_match : bool , # [doc = " The number of transitions in this state."] ntrans : usize , # [doc = " Pairs of input ranges, where there is one pair for each transition."] # [doc = " Each pair specifies an inclusive start and end byte range for the"] # [doc = " corresponding transition."] input_ranges : & 'a [u8] , # [doc = " Transitions to the next state. This slice contains native endian"] # [doc = " encoded state identifiers, with `S` as the representation. Thus, there"] # [doc = " are `ntrans * size_of::<S>()` bytes in this slice."] next : & 'a [u8] , # [doc = " If this is a match state, then this contains the pattern IDs that match"] # [doc = " when the DFA is in this state."] # [doc = ""] # [doc = " This is a contiguous sequence of 32-bit native endian encoded integers."] pattern_ids : & 'a [u8] , # [doc = " An accelerator for this state, if present. If this state has no"] # [doc = " accelerator, then this is an empty slice. When non-empty, this slice"] # [doc = " has length at most 3 and corresponds to the exhaustive set of bytes"] # [doc = " that must be seen in order to transition out of this state."] accel : & 'a [u8] , }
};
}
