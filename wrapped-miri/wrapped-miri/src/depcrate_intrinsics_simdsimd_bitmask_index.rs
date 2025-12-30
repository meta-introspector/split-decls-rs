// Generated macro for simd_bitmask_index (function)
macro_rules! Depcrate_intrinsics_simdsimd_bitmask_index {
() => {
// Module: crate::intrinsics::simd
// Provides: {"simd_bitmask_index"}
// Dependencies: {}
fn simd_bitmask_index (idx : u32 , vec_len : u32 , endianness : Endian) -> u32 { assert ! (idx < vec_len) ; match endianness { Endian :: Little => idx , # [expect (clippy :: arithmetic_side_effects)] Endian :: Big => vec_len - 1 - idx , } }
};
}
