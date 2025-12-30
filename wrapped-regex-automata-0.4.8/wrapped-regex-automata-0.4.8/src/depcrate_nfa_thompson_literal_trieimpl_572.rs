// Generated macro for impl_572 (impl)
macro_rules! Depcrate_nfa_thompson_literal_trieimpl_572 {
() => {
// Module: crate::nfa::thompson::literal_trie
// Provides: {"impl_572"}
// Dependencies: {}
impl < 'a > Frame < 'a > { # [doc = " Create a new stack frame for trie traversal. This initializes the"] # [doc = " 'transitions' iterator to the transitions for the first chunk, with the"] # [doc = " 'chunks' iterator being every chunk after the first one."] fn new (state : & 'a State) -> Frame < 'a > { let mut chunks = state . chunks () ; let chunk = chunks . next () . unwrap () ; let transitions = chunk . iter () ; Frame { chunks , transitions , union : vec ! [] , sparse : vec ! [] } } }
};
}
