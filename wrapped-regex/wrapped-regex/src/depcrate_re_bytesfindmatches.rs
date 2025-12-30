// Generated macro for FindMatches (struct)
macro_rules! Depcrate_re_bytesFindMatches {
() => {
// Module: crate::re_bytes
// Provides: {"FindMatches"}
// Dependencies: {}
# [doc = " An iterator over all non-overlapping matches for a particular string."] # [doc = ""] # [doc = " The iterator yields a tuple of integers corresponding to the start and end"] # [doc = " of the match. The indices are byte offsets. The iterator stops when no more"] # [doc = " matches can be found."] # [doc = ""] # [doc = " `'r` is the lifetime of the compiled regular expression and `'t` is the"] # [doc = " lifetime of the matched byte string."] pub struct FindMatches < 'r , 't > (re_trait :: FindMatches < 't , ExecNoSync < 'r > >) ;
};
}
