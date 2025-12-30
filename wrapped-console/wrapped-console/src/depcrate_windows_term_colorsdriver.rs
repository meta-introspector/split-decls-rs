// Generated macro for driver (function)
macro_rules! Depcrate_windows_term_colorsdriver {
() => {
// Module: crate::windows_term::colors
// Provides: {"driver"}
// Dependencies: {}
fn driver < Out > (parse : fn (Bytes < '_ >) -> Option < Out > , part : & str) -> Option < Out > { let mut bytes = part . bytes () ; loop { while bytes . next () ? != b'\x1b' { } if let ret @ Some (_) = (parse) (bytes . clone ()) { return ret ; } } }
};
}
