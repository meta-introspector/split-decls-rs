// Generated macro for is_valid_cap_letter (function)
macro_rules! Depcrate_expandis_valid_cap_letter {
() => {
// Module: crate::expand
// Provides: {"is_valid_cap_letter"}
// Dependencies: {}
fn is_valid_cap_letter (b : & u8) -> bool { match * b { b'0' ..= b'9' | b'a' ..= b'z' | b'A' ..= b'Z' | b'_' => true , _ => false , } }
};
}
