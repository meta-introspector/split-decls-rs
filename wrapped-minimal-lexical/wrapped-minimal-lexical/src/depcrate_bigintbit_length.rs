// Generated macro for bit_length (function)
macro_rules! Depcrate_bigintbit_length {
() => {
// Module: crate::bigint
// Provides: {"bit_length"}
// Dependencies: {}
# [doc = " Calculate the bit-length of the big-integer."] # [inline] pub fn bit_length (x : & [Limb]) -> u32 { let nlz = leading_zeros (x) ; LIMB_BITS as u32 * x . len () as u32 - nlz }
};
}
