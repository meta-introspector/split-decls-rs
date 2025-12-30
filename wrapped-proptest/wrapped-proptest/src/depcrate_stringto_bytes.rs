// Generated macro for to_bytes (function)
macro_rules! Depcrate_stringto_bytes {
() => {
// Module: crate::string
// Provides: {"to_bytes"}
// Dependencies: {}
fn to_bytes (khar : char) -> Vec < u8 > { let mut buf = [0u8 ; 4] ; khar . encode_utf8 (& mut buf) . as_bytes () . to_owned () }
};
}
