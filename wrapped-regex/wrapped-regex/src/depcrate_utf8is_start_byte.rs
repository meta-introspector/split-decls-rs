// Generated macro for is_start_byte (function)
macro_rules! Depcrate_utf8is_start_byte {
() => {
// Module: crate::utf8
// Provides: {"is_start_byte"}
// Dependencies: {}
fn is_start_byte (b : u8) -> bool { b & 0b11_000000 != 0b1_0000000 }
};
}
