// Generated macro for percent_encode (function)
macro_rules! Depcratepercent_encode {
() => {
// Module: crate
// Provides: {"percent_encode"}
// Dependencies: {}
# [doc = " Percent-encode the given bytes with the given set."] # [doc = ""] # [doc = " Non-ASCII bytes and bytes in `ascii_set` are encoded."] # [doc = ""] # [doc = " The return type:"] # [doc = ""] # [doc = " * Implements `Iterator<Item = &str>` and therefore has a `.collect::<String>()` method,"] # [doc = " * Implements `Display` and therefore has a `.to_string()` method,"] # [doc = " * Implements `Into<Cow<str>>` borrowing `input` when none of its bytes are encoded."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use percent_encoding::{percent_encode, NON_ALPHANUMERIC};"] # [doc = ""] # [doc = " assert_eq!(percent_encode(b\"foo bar?\", NON_ALPHANUMERIC).to_string(), \"foo%20bar%3F\");"] # [doc = " ```"] # [inline] pub fn percent_encode < 'a > (input : & 'a [u8] , ascii_set : & 'static AsciiSet) -> PercentEncode < 'a > { PercentEncode { bytes : input , ascii_set , } }
};
}
