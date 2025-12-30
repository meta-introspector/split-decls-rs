// Generated macro for impl_xoshiro_u32 (macro)
macro_rules! Depcrate_commonimpl_xoshiro_u32 {
() => {
// Module: crate::common
// Provides: {"impl_xoshiro_u32"}
// Dependencies: {}
# [doc = " Implement the xoshiro iteration for `u32` output."] macro_rules ! impl_xoshiro_u32 { ($ self : expr) => { let t = $ self . s [1] << 9 ; $ self . s [2] ^= $ self . s [0] ; $ self . s [3] ^= $ self . s [1] ; $ self . s [1] ^= $ self . s [2] ; $ self . s [0] ^= $ self . s [3] ; $ self . s [2] ^= t ; $ self . s [3] = $ self . s [3] . rotate_left (11) ; } ; }
};
}
