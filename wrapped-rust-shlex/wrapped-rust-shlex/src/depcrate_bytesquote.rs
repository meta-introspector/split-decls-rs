// Generated macro for quote (function)
macro_rules! Depcrate_bytesquote {
() => {
// Module: crate::bytes
// Provides: {"quote"}
// Dependencies: {}
# [doc = " Given a single word, return a string suitable to encode it as a shell argument."] # [doc = ""] # [doc = " Uses default settings except that nul bytes are passed through, which [may be"] # [doc = " dangerous](quoting_warning#nul-bytes), leading to this function being deprecated."] # [doc = ""] # [doc = " Equivalent to [`Quoter::new().allow_nul(true).quote(in_bytes).unwrap()`](Quoter)."] # [doc = ""] # [doc = " (That configuration never returns `Err`, so this function does not panic.)"] # [doc = ""] # [doc = " The string equivalent is [shlex::quote]."] # [deprecated (since = "1.3.0" , note = "replace with `try_quote(str)?` to avoid nul byte danger")] pub fn quote (in_bytes : & [u8]) -> Cow < [u8] > { Quoter :: new () . allow_nul (true) . quote (in_bytes) . unwrap () }
};
}
