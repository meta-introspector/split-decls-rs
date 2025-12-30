// Generated macro for FindIter (struct)
macro_rules! Depcrate_ahocorasickFindIter {
() => {
// Module: crate::ahocorasick
// Provides: {"FindIter"}
// Dependencies: {}
# [doc = " An iterator of non-overlapping matches in a particular haystack."] # [doc = ""] # [doc = " This iterator yields matches according to the [`MatchKind`] used by this"] # [doc = " automaton."] # [doc = ""] # [doc = " This iterator is constructed via the [`AhoCorasick::find_iter`] and"] # [doc = " [`AhoCorasick::try_find_iter`] methods."] # [doc = ""] # [doc = " The lifetime `'a` refers to the lifetime of the `AhoCorasick` automaton."] # [doc = ""] # [doc = " The lifetime `'h` refers to the lifetime of the haystack being searched."] # [derive (Debug)] pub struct FindIter < 'a , 'h > (automaton :: FindIter < 'a , 'h , Arc < dyn AcAutomaton > >) ;
};
}
