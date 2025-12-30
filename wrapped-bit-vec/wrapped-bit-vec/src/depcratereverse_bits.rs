// Generated macro for reverse_bits (function)
macro_rules! Depcratereverse_bits {
() => {
// Module: crate
// Provides: {"reverse_bits"}
// Dependencies: {}
fn reverse_bits (byte : u8) -> u8 { let mut result = 0 ; for i in 0 .. u8 :: bits () { result |= ((byte >> i) & 1) << (u8 :: bits () - 1 - i) ; } result }
};
}
