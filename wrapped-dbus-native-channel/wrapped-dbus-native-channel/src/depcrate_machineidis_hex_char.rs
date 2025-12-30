// Generated macro for is_hex_char (function)
macro_rules! Depcrate_machineidis_hex_char {
() => {
// Module: crate::machineid
// Provides: {"is_hex_char"}
// Dependencies: {}
fn is_hex_char (b : u8) -> bool { match b { b'0' ..= b'9' | b'a' ..= b'f' | b'A' ..= b'F' => true , _ => false , } }
};
}
