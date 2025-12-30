// Generated macro for FindOverlappingIter (struct)
macro_rules! Depcrate_ahocorasickFindOverlappingIter {
() => {
// Module: crate::ahocorasick
// Provides: {"FindOverlappingIter"}
// Dependencies: {}
# [doc = " An iterator of overlapping matches in a particular haystack."] # [doc = ""] # [doc = " This iterator will report all possible matches in a particular haystack,"] # [doc = " even when the matches overlap."] # [doc = ""] # [doc = " This iterator is constructed via the [`AhoCorasick::find_overlapping_iter`]"] # [doc = " and [`AhoCorasick::try_find_overlapping_iter`] methods."] # [doc = ""] # [doc = " The lifetime `'a` refers to the lifetime of the `AhoCorasick` automaton."] # [doc = ""] # [doc = " The lifetime `'h` refers to the lifetime of the haystack being searched."] # [derive (Debug)] pub struct FindOverlappingIter < 'a , 'h > (automaton :: FindOverlappingIter < 'a , 'h , Arc < dyn AcAutomaton > > ,) ;
};
}
