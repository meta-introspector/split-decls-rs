// Generated macro for StateBuilderNFA (struct)
macro_rules! Depcrate_util_determinize_stateStateBuilderNFA {
() => {
// Module: crate::util::determinize::state
// Provides: {"StateBuilderNFA"}
// Dependencies: {}
# [doc = " A state builder that collects some assertions and NFA state IDs."] # [doc = ""] # [doc = " When collecting NFA state IDs is finished, this can be used to build a"] # [doc = " `State` if necessary."] # [doc = ""] # [doc = " When dont with building a state (regardless of whether it got kept or not),"] # [doc = " it's usually a good idea to call `clear` to get an empty builder back so"] # [doc = " that it can be reused to build the next state."] # [derive (Clone)] pub (crate) struct StateBuilderNFA { repr : Vec < u8 > , prev_nfa_state_id : StateID , }
};
}
