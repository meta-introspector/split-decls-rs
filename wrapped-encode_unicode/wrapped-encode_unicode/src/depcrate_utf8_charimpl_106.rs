// Generated macro for impl_106 (impl)
macro_rules! Depcrate_utf8_charimpl_106 {
() => {
// Module: crate::utf8_char
// Provides: {"impl_106"}
// Dependencies: {}
# [cfg (feature = "ascii")] # [doc = " `Utf8Char`s that are not ASCII always compare greater."] impl PartialOrd < Utf8Char > for AsciiChar { # [inline] fn partial_cmp (& self , u8c : & Utf8Char) -> Option < Ordering > { self . partial_cmp (& u8c . bytes [0]) } }
};
}
