// Generated macro for trailing_newline (function)
macro_rules! Depcrate_outputtrailing_newline {
() => {
// Module: crate::output
// Provides: {"trailing_newline"}
// Dependencies: {}
fn trailing_newline (s : & str) -> & str { if s . ends_with ("\r\n") { "\r\n" } else if s . ends_with ('\r') { "\r" } else if s . ends_with ('\n') { "\n" } else { "" } }
};
}
