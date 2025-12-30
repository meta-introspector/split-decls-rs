// Generated macro for starts_with_digit (function)
macro_rules! Depcrate_errorstarts_with_digit {
() => {
// Module: crate::error
// Provides: {"starts_with_digit"}
// Dependencies: {}
fn starts_with_digit (slice : & str) -> bool { match slice . as_bytes () . first () { None => false , Some (& byte) => byte >= b'0' && byte <= b'9' , } }
};
}
