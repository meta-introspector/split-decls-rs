// Generated macro for escape_ascii (function)
macro_rules! Depcrateescape_ascii {
() => {
// Module: crate
// Provides: {"escape_ascii"}
// Dependencies: {}
# [doc = " Converts a byte slice into a string using"] # [doc = " [`core::ascii::escape_default`](https://doc.rust-lang.org/core/ascii/fn.escape_default.html)"] # [doc = " to escape each byte."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " use build_data::escape_ascii;"] # [doc = " assert_eq!(\"abc\", escape_ascii(b\"abc\"));"] # [doc = " assert_eq!(\"abc\\\\n\", escape_ascii(b\"abc\\n\"));"] # [doc = " assert_eq!("] # [doc = "     \"Euro sign: \\\\xe2\\\\x82\\\\xac\","] # [doc = "     escape_ascii(\"Euro sign: \\u{20AC}\".as_bytes())"] # [doc = " );"] # [doc = " assert_eq!(\"\\\\x01\\\\x02\\\\x03\", escape_ascii(&[1, 2, 3]));"] # [doc = " ```"] # [allow (clippy :: missing_panics_doc)] # [must_use] pub fn escape_ascii (input : impl AsRef < [u8] >) -> String { let mut result = String :: new () ; for byte in input . as_ref () { for ascii_byte in core :: ascii :: escape_default (* byte) { result . push_str (core :: str :: from_utf8 (& [ascii_byte]) . unwrap ()) ; } } result }
};
}
