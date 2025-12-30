// Generated macro for StateIter (struct)
macro_rules! Depcrate_dfa_sparseStateIter {
() => {
// Module: crate::dfa::sparse
// Provides: {"StateIter"}
// Dependencies: {}
# [doc = " An iterator over all states in a sparse DFA."] # [doc = ""] # [doc = " This iterator yields tuples, where the first element is the state ID and"] # [doc = " the second element is the state itself."] struct StateIter < 'a , T > { trans : & 'a Transitions < T > , id : usize , }
};
}
