// Generated macro for plusplus_u32 (macro)
macro_rules! Depcrate_commonplusplus_u32 {
() => {
// Module: crate::common
// Provides: {"plusplus_u32"}
// Dependencies: {}
# [doc = " Apply the ++ scrambler used by some RNGs from the xoshiro family."] macro_rules ! plusplus_u32 { ($ x : expr , $ y : expr) => { $ x . wrapping_add ($ y) . rotate_left (7) . wrapping_add ($ x) } ; }
};
}
