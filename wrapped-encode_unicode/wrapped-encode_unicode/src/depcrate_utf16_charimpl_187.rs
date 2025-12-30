// Generated macro for impl_187 (impl)
macro_rules! Depcrate_utf16_charimpl_187 {
() => {
// Module: crate::utf16_char
// Provides: {"impl_187"}
// Dependencies: {}
# [doc = " Only considers the unit equal if the codepoint of the `Utf16Char` is not"] # [doc = " made up of a surrogate pair."] # [doc = ""] # [doc = " There is no impl in the opposite direction, as this should only be used to"] # [doc = " compare `Utf16Char`s against constants."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use encode_unicode::Utf16Char;"] # [doc = " assert!(Utf16Char::from('6') == b'6' as u16);"] # [doc = " assert!(Utf16Char::from('\\u{FFFF}') == 0xffff_u16);"] # [doc = " assert!(Utf16Char::from_tuple((0xd876, Some(0xdef9))).unwrap() != 0xd876_u16);"] # [doc = " ```"] impl PartialEq < u16 > for Utf16Char { fn eq (& self , unit : & u16) -> bool { self . units [0] == * unit && self . units [1] == 0 } }
};
}
