// Generated macro for FindMatches (struct)
macro_rules! Depcrate_meta_regexFindMatches {
() => {
// Module: crate::meta::regex
// Provides: {"FindMatches"}
// Dependencies: {}
# [doc = " An iterator over all non-overlapping matches."] # [doc = ""] # [doc = " The iterator yields a [`Match`] value until no more matches could be found."] # [doc = ""] # [doc = " The lifetime parameters are as follows:"] # [doc = ""] # [doc = " * `'r` represents the lifetime of the `Regex` that produced this iterator."] # [doc = " * `'h` represents the lifetime of the haystack being searched."] # [doc = ""] # [doc = " This iterator can be created with the [`Regex::find_iter`] method."] # [derive (Debug)] pub struct FindMatches < 'r , 'h > { re : & 'r Regex , cache : CachePoolGuard < 'r > , it : iter :: Searcher < 'h > , }
};
}
