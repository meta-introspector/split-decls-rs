// Generated macro for impl_104 (impl)
macro_rules! Depcrate_utf8_charimpl_104 {
() => {
// Module: crate::utf8_char
// Provides: {"impl_104"}
// Dependencies: {}
# [cfg (feature = "ascii")] # [doc = " `Utf8Char`s that are not ASCII never compare equal."] impl PartialEq < Utf8Char > for AsciiChar { # [inline] fn eq (& self , u8c : & Utf8Char) -> bool { u8c == self } }
};
}
