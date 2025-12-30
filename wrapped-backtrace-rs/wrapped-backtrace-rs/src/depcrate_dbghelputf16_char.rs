// Generated macro for utf16_char (function)
macro_rules! Depcrate_dbghelputf16_char {
() => {
// Module: crate::dbghelp
// Provides: {"utf16_char"}
// Dependencies: {}
fn utf16_char (c : char) -> u16 { let buf = & mut [0u16 ; 2] ; let buf = c . encode_utf16 (buf) ; assert ! (buf . len () == 1) ; buf [0] }
};
}
