// Generated macro for low_bits_of_u64 (function)
macro_rules! Depcrate_leb128low_bits_of_u64 {
() => {
// Module: crate::leb128
// Provides: {"low_bits_of_u64"}
// Dependencies: {}
# [inline] # [allow (dead_code)] fn low_bits_of_u64 (val : u64) -> u8 { let byte = val & u64 :: from (u8 :: MAX) ; low_bits_of_byte (byte as u8) }
};
}
