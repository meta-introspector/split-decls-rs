// Generated macro for impl_xoshiro_u64 (macro)
macro_rules! Depcrate_commonimpl_xoshiro_u64 {
() => {
// Module: crate::common
// Provides: {"impl_xoshiro_u64"}
// Dependencies: {}
# [doc = " Implement the xoshiro iteration for `u64` output."] macro_rules ! impl_xoshiro_u64 { ($ self : expr) => { let t = $ self . s [1] << 17 ; $ self . s [2] ^= $ self . s [0] ; $ self . s [3] ^= $ self . s [1] ; $ self . s [1] ^= $ self . s [2] ; $ self . s [0] ^= $ self . s [3] ; $ self . s [2] ^= t ; $ self . s [3] = $ self . s [3] . rotate_left (45) ; } ; }
};
}
