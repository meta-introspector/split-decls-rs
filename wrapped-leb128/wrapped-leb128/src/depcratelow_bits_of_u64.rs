// Generated macro for low_bits_of_u64 (function)
macro_rules! Depcratelow_bits_of_u64 {
() => {
// Module: crate
// Provides: {"low_bits_of_u64"}
// Dependencies: {}
# [doc (hidden)] # [inline] pub fn low_bits_of_u64 (val : u64) -> u8 { let byte = val & (std :: u8 :: MAX as u64) ; low_bits_of_byte (byte as u8) }
};
}
