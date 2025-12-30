// Generated macro for State (struct)
macro_rules! Depcrate_dfa_denseState {
() => {
// Module: crate::dfa::dense
// Provides: {"State"}
// Dependencies: {}
# [doc = " An immutable representation of a single DFA state."] # [doc = ""] # [doc = " `'a` correspondings to the lifetime of a DFA's transition table."] pub (crate) struct State < 'a > { id : StateID , stride2 : usize , transitions : & 'a [StateID] , }
};
}
