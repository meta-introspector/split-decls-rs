// Generated macro for impl_xoroshiro_u32 (macro)
macro_rules! Depcrate_commonimpl_xoroshiro_u32 {
() => {
// Module: crate::common
// Provides: {"impl_xoroshiro_u32"}
// Dependencies: {}
# [doc = " Implement the xoroshiro iteration."] macro_rules ! impl_xoroshiro_u32 { ($ self : expr) => { $ self . s1 ^= $ self . s0 ; $ self . s0 = $ self . s0 . rotate_left (26) ^ $ self . s1 ^ ($ self . s1 << 9) ; $ self . s1 = $ self . s1 . rotate_left (13) ; } ; }
};
}
