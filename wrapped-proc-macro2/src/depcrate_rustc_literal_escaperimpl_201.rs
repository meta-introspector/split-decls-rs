// Generated macro for impl_201 (impl)
macro_rules! Depcrate_rustc_literal_escaperimpl_201 {
() => {
// Module: crate::rustc_literal_escaper
// Provides: {"impl_201"}
// Dependencies: {}
impl EscapeError { # [doc = " Returns true for actual errors, as opposed to warnings."] pub fn is_fatal (& self) -> bool { ! matches ! (self , EscapeError :: UnskippedWhitespaceWarning | EscapeError :: MultipleSkippedLinesWarning) } }
};
}
