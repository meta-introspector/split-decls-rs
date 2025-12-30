// Generated macro for impl_189 (impl)
macro_rules! Depcrate_utf16_charimpl_189 {
() => {
// Module: crate::utf16_char
// Provides: {"impl_189"}
// Dependencies: {}
# [cfg (feature = "ascii")] # [doc = " `Utf16Char`s that are not ASCII never compare equal."] impl PartialEq < AsciiChar > for Utf16Char { # [inline] fn eq (& self , ascii : & AsciiChar) -> bool { self . units [0] == * ascii as u16 } }
};
}
