// Generated macro for next_char (function)
macro_rules! Depcrate_hirnext_char {
() => {
// Module: crate::hir
// Provides: {"next_char"}
// Dependencies: {}
fn next_char (ch : char) -> Option < char > { if ch == '\u{D7FF}' { return Some ('\u{E000}') ; } char :: from_u32 (u32 :: from (ch) . checked_add (1) . unwrap ()) }
};
}
