// Generated macro for impl_175 (impl)
macro_rules! Depcrate_utf16_charimpl_175 {
() => {
// Module: crate::utf16_char
// Provides: {"impl_175"}
// Dependencies: {}
# [cfg (feature = "ascii")] # [doc = " Requires the feature \"ascii\"."] impl ToAsciiChar for Utf16Char { # [inline] fn to_ascii_char (self) -> Result < AsciiChar , ToAsciiCharError > { self . units [0] . to_ascii_char () } # [inline] unsafe fn to_ascii_char_unchecked (self) -> AsciiChar { unsafe { self . units [0] . to_ascii_char_unchecked () } } }
};
}
