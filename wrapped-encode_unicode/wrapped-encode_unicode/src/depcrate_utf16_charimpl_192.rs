// Generated macro for impl_192 (impl)
macro_rules! Depcrate_utf16_charimpl_192 {
() => {
// Module: crate::utf16_char
// Provides: {"impl_192"}
// Dependencies: {}
# [cfg (feature = "ascii")] # [doc = " `Utf16Char`s that are not ASCII always compare greater."] impl PartialOrd < Utf16Char > for AsciiChar { # [inline] fn partial_cmp (& self , u16c : & Utf16Char) -> Option < Ordering > { (* self as u16) . partial_cmp (& u16c . units [0]) } }
};
}
