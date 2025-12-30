// Generated macro for impl_xoroshiro_u64 (macro)
macro_rules! Depcrate_commonimpl_xoroshiro_u64 {
() => {
// Module: crate::common
// Provides: {"impl_xoroshiro_u64"}
// Dependencies: {}
# [doc = " Implement the xoroshiro iteration."] macro_rules ! impl_xoroshiro_u64 { ($ self : expr) => { $ self . s1 ^= $ self . s0 ; $ self . s0 = $ self . s0 . rotate_left (24) ^ $ self . s1 ^ ($ self . s1 << 16) ; $ self . s1 = $ self . s1 . rotate_left (37) ; } ; }
};
}
