// Generated macro for impl_191 (impl)
macro_rules! Depcrate_utf16_charimpl_191 {
() => {
// Module: crate::utf16_char
// Provides: {"impl_191"}
// Dependencies: {}
# [cfg (feature = "ascii")] # [doc = " `Utf16Char`s that are not ASCII always compare greater."] impl PartialOrd < AsciiChar > for Utf16Char { # [inline] fn partial_cmp (& self , ascii : & AsciiChar) -> Option < Ordering > { self . units [0] . partial_cmp (& (* ascii as u16)) } }
};
}
