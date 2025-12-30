// Generated macro for impl_174 (impl)
macro_rules! Depcrate_utf16_charimpl_174 {
() => {
// Module: crate::utf16_char
// Provides: {"impl_174"}
// Dependencies: {}
# [cfg (feature = "ascii")] # [doc = " Requires the feature \"ascii\"."] impl From < AsciiChar > for Utf16Char { # [inline] fn from (ac : AsciiChar) -> Self { Utf16Char { units : [ac . as_byte () as u16 , 0] } } }
};
}
