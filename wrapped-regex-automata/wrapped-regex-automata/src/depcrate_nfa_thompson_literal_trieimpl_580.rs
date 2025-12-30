// Generated macro for impl_580 (impl)
macro_rules! Depcrate_nfa_thompson_literal_trieimpl_580 {
() => {
// Module: crate::nfa::thompson::literal_trie
// Provides: {"impl_580"}
// Dependencies: {}
impl < 'a > Iterator for StateChunksIter < 'a > { type Item = & 'a [Transition] ; fn next (& mut self) -> Option < & 'a [Transition] > { if let Some (& (start , end)) = self . chunks . next () { return Some (& self . transitions [start .. end]) ; } if let Some (chunk) = self . active . take () { return Some (chunk) ; } None } }
};
}
