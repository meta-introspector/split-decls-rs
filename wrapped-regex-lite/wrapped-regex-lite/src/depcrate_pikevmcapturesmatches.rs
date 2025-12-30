// Generated macro for CapturesMatches (struct)
macro_rules! Depcrate_pikevmCapturesMatches {
() => {
// Module: crate::pikevm
// Provides: {"CapturesMatches"}
// Dependencies: {}
# [doc = " An iterator over all successive non-overlapping capture matches in a particular"] # [doc = " haystack. `'r` represents the lifetime of the regex, `'c` is the lifetime"] # [doc = " of the cache and `'h` represents the lifetime of the haystack."] # [derive (Debug)] pub (crate) struct CapturesMatches < 'r , 'h > { it : FindMatches < 'r , 'h > , }
};
}
