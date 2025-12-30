// Generated macro for impl_92 (impl)
macro_rules! Depcrate_utf8_charimpl_92 {
() => {
// Module: crate::utf8_char
// Provides: {"impl_92"}
// Dependencies: {}
# [cfg (feature = "ascii")] # [doc = " Requires the feature \"ascii\"."] impl ToAsciiChar for Utf8Char { fn to_ascii_char (self) -> Result < AsciiChar , ToAsciiCharError > { self . bytes [0] . to_ascii_char () } unsafe fn to_ascii_char_unchecked (self) -> AsciiChar { unsafe { self . bytes [0] . to_ascii_char_unchecked () } } }
};
}
