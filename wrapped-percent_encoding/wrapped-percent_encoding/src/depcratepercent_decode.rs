// Generated macro for percent_decode (function)
macro_rules! Depcratepercent_decode {
() => {
// Module: crate
// Provides: {"percent_decode"}
// Dependencies: {}
# [doc = " Percent-decode the given bytes."] # [doc = ""] # [doc = " <https://url.spec.whatwg.org/#percent-decode>"] # [doc = ""] # [doc = " Any sequence of `%` followed by two hexadecimal digits is decoded."] # [doc = " The return type:"] # [doc = ""] # [doc = " * Implements `Into<Cow<u8>>` borrowing `input` when it contains no percent-encoded sequence,"] # [doc = " * Implements `Iterator<Item = u8>` and therefore has a `.collect::<Vec<u8>>()` method,"] # [doc = " * Has `decode_utf8()` and `decode_utf8_lossy()` methods."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use percent_encoding::percent_decode;"] # [doc = ""] # [doc = " assert_eq!(percent_decode(b\"foo%20bar%3f\").decode_utf8().unwrap(), \"foo bar?\");"] # [doc = " ```"] # [inline] pub fn percent_decode (input : & [u8]) -> PercentDecode < '_ > { PercentDecode { bytes : input . iter () , } }
};
}
