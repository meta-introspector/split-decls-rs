// Generated macro for Mask (struct)
macro_rules! Depcrate_simd_accel_teddy128Mask {
() => {
// Module: crate::simd_accel::teddy128
// Provides: {"Mask"}
// Dependencies: {}
# [doc = " A single mask."] # [derive (Debug , Clone , Copy)] struct Mask { # [doc = " Bitsets for the low nybbles in a fingerprint."] lo : u8x16 , # [doc = " Bitsets for the high nybbles in a fingerprint."] hi : u8x16 , }
};
}
