// Generated macro for impl_79 (impl)
macro_rules! Depcrate_line_endingimpl_79 {
() => {
// Module: crate::line_ending
// Provides: {"impl_79"}
// Dependencies: {}
# [allow (clippy :: len_without_is_empty)] impl LineEnding { # [doc = " Get the byte serialization of this [`LineEnding`]."] pub fn as_bytes (self) -> & 'static [u8] { match self { LineEnding :: CR => & [CHAR_CR] , LineEnding :: LF => & [CHAR_LF] , LineEnding :: CRLF => & [CHAR_CR , CHAR_LF] , } } # [doc = " Get the encoded length of this [`LineEnding`]."] pub fn len (self) -> usize { self . as_bytes () . len () } }
};
}
