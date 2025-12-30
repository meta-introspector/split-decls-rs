// Generated macro for impl_105 (impl)
macro_rules! Depcrate_utf8_charimpl_105 {
() => {
// Module: crate::utf8_char
// Provides: {"impl_105"}
// Dependencies: {}
# [cfg (feature = "ascii")] # [doc = " `Utf8Char`s that are not ASCII always compare greater."] impl PartialOrd < AsciiChar > for Utf8Char { # [inline] fn partial_cmp (& self , ascii : & AsciiChar) -> Option < Ordering > { self . bytes [0] . partial_cmp (ascii) } }
};
}
