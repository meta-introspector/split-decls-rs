// Generated macro for StateChunksIter (struct)
macro_rules! Depcrate_nfa_thompson_literal_trieStateChunksIter {
() => {
// Module: crate::nfa::thompson::literal_trie
// Provides: {"StateChunksIter"}
// Dependencies: {}
# [doc = " An iterator over all of the chunks in a state, including the active chunk."] # [doc = ""] # [doc = " This iterator is created by `State::chunks`. We name this iterator so that"] # [doc = " we can include it in the `Frame` type for non-recursive trie traversal."] # [derive (Debug)] struct StateChunksIter < 'a > { transitions : & 'a [Transition] , chunks : core :: slice :: Iter < 'a , (usize , usize) > , active : Option < & 'a [Transition] > , }
};
}
