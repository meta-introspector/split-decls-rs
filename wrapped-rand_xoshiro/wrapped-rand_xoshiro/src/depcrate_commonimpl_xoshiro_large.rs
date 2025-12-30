// Generated macro for impl_xoshiro_large (macro)
macro_rules! Depcrate_commonimpl_xoshiro_large {
() => {
// Module: crate::common
// Provides: {"impl_xoshiro_large"}
// Dependencies: {}
# [doc = " Implement the large-state xoshiro iteration."] macro_rules ! impl_xoshiro_large { ($ self : expr) => { let t = $ self . s [1] << 11 ; $ self . s [2] ^= $ self . s [0] ; $ self . s [5] ^= $ self . s [1] ; $ self . s [1] ^= $ self . s [2] ; $ self . s [7] ^= $ self . s [3] ; $ self . s [3] ^= $ self . s [4] ; $ self . s [4] ^= $ self . s [5] ; $ self . s [0] ^= $ self . s [6] ; $ self . s [6] ^= $ self . s [7] ; $ self . s [6] ^= t ; $ self . s [7] = $ self . s [7] . rotate_left (21) ; } ; }
};
}
