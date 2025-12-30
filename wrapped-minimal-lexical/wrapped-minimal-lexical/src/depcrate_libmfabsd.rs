// Generated macro for fabsd (function)
macro_rules! Depcrate_libmfabsd {
() => {
// Module: crate::libm
// Provides: {"fabsd"}
// Dependencies: {}
# [doc = " Absolute value (magnitude) (f64)"] # [doc = " Calculates the absolute value (magnitude) of the argument `x`,"] # [doc = " by direct manipulation of the bit representation of `x`."] pub fn fabsd (x : f64) -> f64 { f64 :: from_bits (x . to_bits () & (u64 :: MAX / 2)) }
};
}
