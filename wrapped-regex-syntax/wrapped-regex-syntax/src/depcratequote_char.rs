// Generated macro for quote_char (function)
macro_rules! Depcratequote_char {
() => {
// Module: crate
// Provides: {"quote_char"}
// Dependencies: {}
fn quote_char (c : char) -> String { let mut s = String :: new () ; if parser :: is_punct (c) { s . push ('\\') ; } s . push (c) ; s }
};
}
