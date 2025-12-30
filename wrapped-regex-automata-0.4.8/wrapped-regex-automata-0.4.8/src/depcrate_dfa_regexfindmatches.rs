// Generated macro for FindMatches (struct)
macro_rules! Depcrate_dfa_regexFindMatches {
() => {
// Module: crate::dfa::regex
// Provides: {"FindMatches"}
// Dependencies: {}
# [doc = " An iterator over all non-overlapping matches for an infallible search."] # [doc = ""] # [doc = " The iterator yields a [`Match`] value until no more matches could be found."] # [doc = " If the underlying regex engine returns an error, then a panic occurs."] # [doc = ""] # [doc = " The type parameters are as follows:"] # [doc = ""] # [doc = " * `A` represents the type of the underlying DFA that implements the"] # [doc = " [`Automaton`] trait."] # [doc = ""] # [doc = " The lifetime parameters are as follows:"] # [doc = ""] # [doc = " * `'h` represents the lifetime of the haystack being searched."] # [doc = " * `'r` represents the lifetime of the regex object itself."] # [doc = ""] # [doc = " This iterator can be created with the [`Regex::find_iter`] method."] # [derive (Debug)] pub struct FindMatches < 'r , 'h , A > { re : & 'r Regex < A > , it : iter :: Searcher < 'h > , }
};
}
