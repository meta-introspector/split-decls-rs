// Generated macro for FindCaptures (struct)
macro_rules! Depcrate_re_unicodeFindCaptures {
() => {
// Module: crate::re_unicode
// Provides: {"FindCaptures"}
// Dependencies: {}
# [doc = " An iterator that yields all non-overlapping capture groups matching a"] # [doc = " particular regular expression."] # [doc = ""] # [doc = " The iterator stops when no more matches can be found."] # [doc = ""] # [doc = " `'r` is the lifetime of the compiled regular expression and `'t` is the"] # [doc = " lifetime of the matched string."] pub struct FindCaptures < 'r , 't > (FindCapturesInner < 'r , 't >) ;
};
}
