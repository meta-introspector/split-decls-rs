// Generated macro for http_whitespace (function)
macro_rules! Depcrate_mimehttp_whitespace {
() => {
// Module: crate::mime
// Provides: {"http_whitespace"}
// Dependencies: {}
fn http_whitespace (c : char) -> bool { matches ! (c , ' ' | '\t' | '\n' | '\r') }
};
}
