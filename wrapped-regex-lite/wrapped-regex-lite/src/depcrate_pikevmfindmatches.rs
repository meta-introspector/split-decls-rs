// Generated macro for FindMatches (struct)
macro_rules! Depcrate_pikevmFindMatches {
() => {
// Module: crate::pikevm
// Provides: {"FindMatches"}
// Dependencies: {}
# [doc = " An iterator over all successive non-overlapping matches in a particular"] # [doc = " haystack. `'r` represents the lifetime of the regex, `'c` is the lifetime"] # [doc = " of the cache and `'h` represents the lifetime of the haystack."] # [derive (Debug)] pub (crate) struct FindMatches < 'r , 'h > { pikevm : & 'r PikeVM , cache : CachePoolGuard < 'r > , haystack : & 'h [u8] , at : usize , slots : Vec < Option < NonMaxUsize > > , last_match_end : Option < usize > , }
};
}
