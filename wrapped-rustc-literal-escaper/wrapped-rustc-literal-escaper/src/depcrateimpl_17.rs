// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl EscapeError { # [doc = " Returns true for actual errors, as opposed to warnings."] pub fn is_fatal (& self) -> bool { ! matches ! (self , EscapeError :: UnskippedWhitespaceWarning | EscapeError :: MultipleSkippedLinesWarning) } }
};
}
