// Generated macro for starstar_u32 (macro)
macro_rules! Depcrate_commonstarstar_u32 {
() => {
// Module: crate::common
// Provides: {"starstar_u32"}
// Dependencies: {}
# [doc = " Apply the ** scrambler used by some RNGs from the xoshiro family."] macro_rules ! starstar_u32 { ($ x : expr) => { $ x . wrapping_mul (0x9E3779BB) . rotate_left (5) . wrapping_mul (5) } ; }
};
}
