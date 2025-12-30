// Generated macro for FindMatches (struct)
macro_rules! Depcrate_hybrid_regexFindMatches {
() => {
// Module: crate::hybrid::regex
// Provides: {"FindMatches"}
// Dependencies: {}
# [doc = " An iterator over all non-overlapping matches for an infallible search."] # [doc = ""] # [doc = " The iterator yields a [`Match`] value until no more matches could be found."] # [doc = " If the underlying regex engine returns an error, then a panic occurs."] # [doc = ""] # [doc = " The lifetime parameters are as follows:"] # [doc = ""] # [doc = " * `'r` represents the lifetime of the regex object."] # [doc = " * `'h` represents the lifetime of the haystack being searched."] # [doc = " * `'c` represents the lifetime of the regex cache."] # [doc = ""] # [doc = " This iterator can be created with the [`Regex::find_iter`] method."] # [derive (Debug)] pub struct FindMatches < 'r , 'c , 'h > { re : & 'r Regex , cache : & 'c mut Cache , it : iter :: Searcher < 'h > , }
};
}
