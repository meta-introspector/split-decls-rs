// Generated macro for StateBuilderEmpty (struct)
macro_rules! Depcrate_util_determinize_stateStateBuilderEmpty {
() => {
// Module: crate::util::determinize::state
// Provides: {"StateBuilderEmpty"}
// Dependencies: {}
# [doc = " A state builder that represents an empty state."] # [doc = ""] # [doc = " This is a useful \"initial condition\" for state construction. It has no"] # [doc = " NFA state IDs, no assertions set and no pattern IDs. No allocations are"] # [doc = " made when new() is called. Its main use is for being converted into a"] # [doc = " builder that can capture assertions and pattern IDs."] # [derive (Clone , Debug)] pub (crate) struct StateBuilderEmpty (Vec < u8 >) ;
};
}
