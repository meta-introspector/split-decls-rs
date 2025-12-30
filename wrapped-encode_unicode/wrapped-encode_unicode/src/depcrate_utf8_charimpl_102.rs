// Generated macro for impl_102 (impl)
macro_rules! Depcrate_utf8_charimpl_102 {
() => {
// Module: crate::utf8_char
// Provides: {"impl_102"}
// Dependencies: {}
# [doc = " Only considers the byte equal if both it and the `Utf8Char` represents ASCII characters."] # [doc = ""] # [doc = " There is no impl in the opposite direction, as this should only be used to"] # [doc = " compare `Utf8Char`s against constants."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use encode_unicode::Utf8Char;"] # [doc = " assert!(Utf8Char::from('8') == b'8');"] # [doc = " assert!(Utf8Char::from_array([0xf1,0x80,0x80,0x80]).unwrap() != 0xf1);"] # [doc = " assert!(Utf8Char::from('\\u{ff}') != 0xff);"] # [doc = " assert!(Utf8Char::from('\\u{80}') != 0x80);"] # [doc = " ```"] impl PartialEq < u8 > for Utf8Char { fn eq (& self , byte : & u8) -> bool { self . bytes [0] == * byte && self . bytes [1] == 0 } }
};
}
