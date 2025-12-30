// Generated macro for FindCaptures (struct)
macro_rules! Depcrate_re_bytesFindCaptures {
() => {
// Module: crate::re_bytes
// Provides: {"FindCaptures"}
// Dependencies: {}
# [doc = " An iterator that yields all non-overlapping capture groups matching a"] # [doc = " particular regular expression."] # [doc = ""] # [doc = " The iterator stops when no more matches can be found."] # [doc = ""] # [doc = " `'r` is the lifetime of the compiled regular expression and `'t` is the"] # [doc = " lifetime of the matched byte string."] pub struct FindCaptures < 'r , 't > (re_trait :: FindCaptures < 't , ExecNoSync < 'r > >) ;
};
}
