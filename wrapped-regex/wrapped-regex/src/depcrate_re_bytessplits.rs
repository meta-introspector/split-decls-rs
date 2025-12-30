// Generated macro for Splits (struct)
macro_rules! Depcrate_re_bytesSplits {
() => {
// Module: crate::re_bytes
// Provides: {"Splits"}
// Dependencies: {}
# [doc = " Yields all substrings delimited by a regular expression match."] # [doc = ""] # [doc = " `'r` is the lifetime of the compiled regular expression and `'t` is the"] # [doc = " lifetime of the byte string being split."] pub struct Splits < 'r , 't > { finder : FindMatches < 'r , 't > , last : usize , }
};
}
