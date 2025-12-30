// Generated macro for plusplus_u64 (macro)
macro_rules! Depcrate_commonplusplus_u64 {
() => {
// Module: crate::common
// Provides: {"plusplus_u64"}
// Dependencies: {}
# [doc = " Apply the ++ scrambler used by some RNGs from the xoshiro family."] macro_rules ! plusplus_u64 { ($ x : expr , $ y : expr , $ rot : expr) => { $ x . wrapping_add ($ y) . rotate_left ($ rot) . wrapping_add ($ x) } ; }
};
}
