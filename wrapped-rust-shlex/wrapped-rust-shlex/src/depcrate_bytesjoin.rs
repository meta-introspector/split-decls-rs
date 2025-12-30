// Generated macro for join (function)
macro_rules! Depcrate_bytesjoin {
() => {
// Module: crate::bytes
// Provides: {"join"}
// Dependencies: {}
# [doc = " Convenience function that consumes an iterable of words and turns it into a single byte string,"] # [doc = " quoting words when necessary. Consecutive words will be separated by a single space."] # [doc = ""] # [doc = " Uses default settings except that nul bytes are passed through, which [may be"] # [doc = " dangerous](quoting_warning#nul-bytes), leading to this function being deprecated."] # [doc = ""] # [doc = " Equivalent to [`Quoter::new().allow_nul(true).join(words).unwrap()`](Quoter)."] # [doc = ""] # [doc = " (That configuration never returns `Err`, so this function does not panic.)"] # [doc = ""] # [doc = " The string equivalent is [shlex::join]."] # [deprecated (since = "1.3.0" , note = "replace with `try_join(words)?` to avoid nul byte danger")] pub fn join < 'a , I : IntoIterator < Item = & 'a [u8] > > (words : I) -> Vec < u8 > { Quoter :: new () . allow_nul (true) . join (words) . unwrap () }
};
}
