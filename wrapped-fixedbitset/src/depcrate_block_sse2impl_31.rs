// Generated macro for impl_31 (impl)
macro_rules! Depcrate_block_sse2impl_31 {
() => {
// Module: crate::block::sse2
// Provides: {"impl_31"}
// Dependencies: {}
impl PartialEq for Block { # [inline] fn eq (& self , other : & Self) -> bool { unsafe { # [cfg (not (target_feature = "sse4.1"))] { _mm_movemask_epi8 (_mm_cmpeq_epi8 (self . 0 , other . 0)) == 0xffff } # [cfg (target_feature = "sse4.1")] { let neq = _mm_xor_si128 (self . 0 , other . 0) ; _mm_test_all_zeros (neq , neq) == 1 } } } }
};
}
