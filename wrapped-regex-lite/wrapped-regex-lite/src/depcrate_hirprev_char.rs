// Generated macro for prev_char (function)
macro_rules! Depcrate_hirprev_char {
() => {
// Module: crate::hir
// Provides: {"prev_char"}
// Dependencies: {}
fn prev_char (ch : char) -> Option < char > { if ch == '\u{E000}' { return Some ('\u{D7FF}') ; } Some (char :: from_u32 (u32 :: from (ch) . checked_sub (1) ?) . unwrap ()) }
};
}
