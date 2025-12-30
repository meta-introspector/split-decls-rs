// Generated macro for impl_23 (impl)
macro_rules! Depcrate_block_sse2impl_23 {
() => {
// Module: crate::block::sse2
// Provides: {"impl_23"}
// Dependencies: {}
impl Block { # [inline] pub fn is_empty (self) -> bool { # [cfg (not (target_feature = "sse4.1"))] { self == Self :: NONE } # [cfg (target_feature = "sse4.1")] { unsafe { _mm_test_all_zeros (self . 0 , self . 0) == 1 } } } # [inline] pub fn andnot (self , other : Self) -> Self { Self (unsafe { _mm_andnot_si128 (other . 0 , self . 0) }) } }
};
}
