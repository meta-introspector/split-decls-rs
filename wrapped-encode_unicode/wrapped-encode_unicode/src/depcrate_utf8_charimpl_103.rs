// Generated macro for impl_103 (impl)
macro_rules! Depcrate_utf8_charimpl_103 {
() => {
// Module: crate::utf8_char
// Provides: {"impl_103"}
// Dependencies: {}
# [cfg (feature = "ascii")] # [doc = " `Utf8Char`s that are not ASCII never compare equal."] impl PartialEq < AsciiChar > for Utf8Char { # [inline] fn eq (& self , ascii : & AsciiChar) -> bool { self . bytes [0] == * ascii as u8 } }
};
}
