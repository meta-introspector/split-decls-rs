// Generated macro for SplitsN (struct)
macro_rules! Depcrate_re_bytesSplitsN {
() => {
// Module: crate::re_bytes
// Provides: {"SplitsN"}
// Dependencies: {}
# [doc = " Yields at most `N` substrings delimited by a regular expression match."] # [doc = ""] # [doc = " The last substring will be whatever remains after splitting."] # [doc = ""] # [doc = " `'r` is the lifetime of the compiled regular expression and `'t` is the"] # [doc = " lifetime of the byte string being split."] pub struct SplitsN < 'r , 't > { splits : Splits < 'r , 't > , n : usize , }
};
}
