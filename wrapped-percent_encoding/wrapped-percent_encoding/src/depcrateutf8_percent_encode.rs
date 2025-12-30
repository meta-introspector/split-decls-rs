// Generated macro for utf8_percent_encode (function)
macro_rules! Depcrateutf8_percent_encode {
() => {
// Module: crate
// Provides: {"utf8_percent_encode"}
// Dependencies: {}
# [doc = " Percent-encode the UTF-8 encoding of the given string."] # [doc = ""] # [doc = " See [`percent_encode`] regarding the return type."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};"] # [doc = ""] # [doc = " assert_eq!(utf8_percent_encode(\"foo bar?\", NON_ALPHANUMERIC).to_string(), \"foo%20bar%3F\");"] # [doc = " ```"] # [inline] pub fn utf8_percent_encode < 'a > (input : & 'a str , ascii_set : & 'static AsciiSet) -> PercentEncode < 'a > { percent_encode (input . as_bytes () , ascii_set) }
};
}
