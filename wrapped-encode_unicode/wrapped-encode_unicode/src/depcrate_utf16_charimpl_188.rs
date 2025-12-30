// Generated macro for impl_188 (impl)
macro_rules! Depcrate_utf16_charimpl_188 {
() => {
// Module: crate::utf16_char
// Provides: {"impl_188"}
// Dependencies: {}
# [doc = " Only considers the byte equal if the codepoint of the `Utf16Char` is <= U+FF."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use encode_unicode::Utf16Char;"] # [doc = " assert!(Utf16Char::from('6') == b'6');"] # [doc = " assert!(Utf16Char::from('\\u{00FF}') == b'\\xff');"] # [doc = " assert!(Utf16Char::from('\\u{0100}') != b'\\0');"] # [doc = " ```"] impl PartialEq < u8 > for Utf16Char { fn eq (& self , byte : & u8) -> bool { self . units [0] == * byte as u16 } }
};
}
