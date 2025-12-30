// Generated macro for FindIter (struct)
macro_rules! Depcrate_automatonFindIter {
() => {
// Module: crate::automaton
// Provides: {"FindIter"}
// Dependencies: {}
# [doc = " An iterator of non-overlapping matches in a particular haystack."] # [doc = ""] # [doc = " This iterator yields matches according to the [`MatchKind`] used by this"] # [doc = " automaton."] # [doc = ""] # [doc = " This iterator is constructed via the [`Automaton::try_find_iter`] method."] # [doc = ""] # [doc = " The type variable `A` refers to the implementation of the [`Automaton`]"] # [doc = " trait used to execute the search."] # [doc = ""] # [doc = " The lifetime `'a` refers to the lifetime of the [`Automaton`]"] # [doc = " implementation."] # [doc = ""] # [doc = " The lifetime `'h` refers to the lifetime of the haystack being searched."] # [derive (Debug)] pub struct FindIter < 'a , 'h , A > { # [doc = " The automaton used to drive the search."] aut : & 'a A , # [doc = " The input parameters to give to each search call."] # [doc = ""] # [doc = " The start position of the search is mutated during iteration."] input : Input < 'h > , # [doc = " Records the end offset of the most recent match. This is necessary to"] # [doc = " handle a corner case for preventing empty matches from overlapping with"] # [doc = " the ending bounds of a prior match."] last_match_end : Option < usize > , }
};
}
