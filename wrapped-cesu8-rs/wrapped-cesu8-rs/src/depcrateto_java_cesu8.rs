// Generated macro for to_java_cesu8 (function)
macro_rules! Depcrateto_java_cesu8 {
() => {
// Module: crate
// Provides: {"to_java_cesu8"}
// Dependencies: {}
# [doc = " Convert a Rust `&str` to Java's modified UTF-8 bytes."] # [doc = ""] # [doc = " ```"] # [doc = " use std::borrow::Cow;"] # [doc = " use cesu8::to_java_cesu8;"] # [doc = ""] # [doc = " // This string is valid as UTF-8 or CESU-8, so it doesn't change,"] # [doc = " // and we can convert it without allocating memory."] # [doc = " assert_eq!(Cow::Borrowed(\"aé日\".as_bytes()), to_java_cesu8(\"aé日\"));"] # [doc = ""] # [doc = " // This string is a 4-byte UTF-8 string, which becomes a 6-byte modified"] # [doc = " // UTF-8 vector."] # [doc = " assert_eq!(Cow::Borrowed(&[0xED, 0xA0, 0x81, 0xED, 0xB0, 0x81]),"] # [doc = "            to_java_cesu8(\"\\u{10401}\"));"] # [doc = ""] # [doc = " // This string contains null, which becomes 2-byte modified UTF-8 encoding"] # [doc = " assert_eq!(Cow::Borrowed(&[0xC0, 0x80, 0xC0, 0x80]),"] # [doc = "            to_java_cesu8(\"\\0\\0\"));"] # [doc = " ```"] pub fn to_java_cesu8 (text : & str) -> Cow < [u8] > { if is_valid_java_cesu8 (text) { Cow :: Borrowed (text . as_bytes ()) } else { Cow :: Owned (to_cesu8_internal (text , Variant :: Java)) } }
};
}
