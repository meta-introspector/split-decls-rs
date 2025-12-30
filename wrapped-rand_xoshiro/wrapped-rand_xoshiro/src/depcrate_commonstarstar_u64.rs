// Generated macro for starstar_u64 (macro)
macro_rules! Depcrate_commonstarstar_u64 {
() => {
// Module: crate::common
// Provides: {"starstar_u64"}
// Dependencies: {}
# [doc = " Apply the ** scrambler used by some RNGs from the xoshiro family."] macro_rules ! starstar_u64 { ($ x : expr) => { $ x . wrapping_mul (5) . rotate_left (7) . wrapping_mul (9) } ; }
};
}
