// Generated macro for impl_xoroshiro_u64_plusplus (macro)
macro_rules! Depcrate_commonimpl_xoroshiro_u64_plusplus {
() => {
// Module: crate::common
// Provides: {"impl_xoroshiro_u64_plusplus"}
// Dependencies: {}
# [doc = " Implement the xoroshiro iteration for the ++ scrambler."] macro_rules ! impl_xoroshiro_u64_plusplus { ($ self : expr) => { $ self . s1 ^= $ self . s0 ; $ self . s0 = $ self . s0 . rotate_left (49) ^ $ self . s1 ^ ($ self . s1 << 21) ; $ self . s1 = $ self . s1 . rotate_left (28) ; } ; }
};
}
