// Generated macro for impl_91 (impl)
macro_rules! Depcrate_utf8_charimpl_91 {
() => {
// Module: crate::utf8_char
// Provides: {"impl_91"}
// Dependencies: {}
# [cfg (feature = "ascii")] # [doc = " Requires the feature \"ascii\"."] impl From < AsciiChar > for Utf8Char { fn from (ac : AsciiChar) -> Self { Utf8Char { bytes : [ac . as_byte () , 0 , 0 , 0] } } }
};
}
