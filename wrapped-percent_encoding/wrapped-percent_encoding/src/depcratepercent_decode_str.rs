// Generated macro for percent_decode_str (function)
macro_rules! Depcratepercent_decode_str {
() => {
// Module: crate
// Provides: {"percent_decode_str"}
// Dependencies: {}
# [doc = " Percent-decode the given string."] # [doc = ""] # [doc = " <https://url.spec.whatwg.org/#string-percent-decode>"] # [doc = ""] # [doc = " See [`percent_decode`] regarding the return type."] # [inline] pub fn percent_decode_str (input : & str) -> PercentDecode < '_ > { percent_decode (input . as_bytes ()) }
};
}
