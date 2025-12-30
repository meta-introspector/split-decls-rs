// Generated macro for impl_190 (impl)
macro_rules! Depcrate_utf16_charimpl_190 {
() => {
// Module: crate::utf16_char
// Provides: {"impl_190"}
// Dependencies: {}
# [cfg (feature = "ascii")] # [doc = " `Utf16Char`s that are not ASCII never compare equal."] impl PartialEq < Utf16Char > for AsciiChar { # [inline] fn eq (& self , u16c : & Utf16Char) -> bool { * self as u16 == u16c . units [0] } }
};
}
