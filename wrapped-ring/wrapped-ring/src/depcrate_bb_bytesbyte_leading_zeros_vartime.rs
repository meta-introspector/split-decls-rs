// Generated macro for byte_leading_zeros_vartime (function)
macro_rules! Depcrate_bb_bytesbyte_leading_zeros_vartime {
() => {
// Module: crate::bb::bytes
// Provides: {"byte_leading_zeros_vartime"}
// Dependencies: {}
pub fn byte_leading_zeros_vartime (a : & u8) -> BitLength < usize > { BitLength :: from_bits (usize_from_u32 (a . leading_zeros ())) }
};
}
