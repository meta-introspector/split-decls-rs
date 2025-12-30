// Generated macro for State (struct)
macro_rules! Depcrate_nfa_contiguousState {
() => {
// Module: crate::nfa::contiguous
// Provides: {"State"}
// Dependencies: {}
# [doc = " The \"in memory\" representation a single dense or sparse state."] # [doc = ""] # [doc = " A `State`'s in memory representation is not ever actually materialized"] # [doc = " during a search with a contiguous NFA. Doing so would be too slow. (Indeed,"] # [doc = " the only time a `State` is actually constructed is in `Debug` impls.)"] # [doc = " Instead, a `State` exposes a number of static methods for reading certain"] # [doc = " things from the raw binary encoding of the state."] # [derive (Clone)] struct State < 'a > { # [doc = " The state to transition to when 'class_to_next' yields a transition"] # [doc = " to the FAIL state."] fail : StateID , # [doc = " The number of pattern IDs in this state. For a non-match state, this is"] # [doc = " always zero. Otherwise it is always bigger than zero."] match_len : usize , # [doc = " The sparse or dense representation of the transitions for this state."] trans : StateTrans < 'a > , }
};
}
