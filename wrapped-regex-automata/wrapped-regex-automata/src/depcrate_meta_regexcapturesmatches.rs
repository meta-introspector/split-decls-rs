// Generated macro for CapturesMatches (struct)
macro_rules! Depcrate_meta_regexCapturesMatches {
() => {
// Module: crate::meta::regex
// Provides: {"CapturesMatches"}
// Dependencies: {}
# [doc = " An iterator over all non-overlapping leftmost matches with their capturing"] # [doc = " groups."] # [doc = ""] # [doc = " The iterator yields a [`Captures`] value until no more matches could be"] # [doc = " found."] # [doc = ""] # [doc = " The lifetime parameters are as follows:"] # [doc = ""] # [doc = " * `'r` represents the lifetime of the `Regex` that produced this iterator."] # [doc = " * `'h` represents the lifetime of the haystack being searched."] # [doc = ""] # [doc = " This iterator can be created with the [`Regex::captures_iter`] method."] # [derive (Debug)] pub struct CapturesMatches < 'r , 'h > { re : & 'r Regex , cache : CachePoolGuard < 'r > , caps : Captures , it : iter :: Searcher < 'h > , }
};
}
