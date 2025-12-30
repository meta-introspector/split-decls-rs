// Generated macro for impl_61 (impl)
macro_rules! Depcrate_block_avx2impl_61 {
() => {
// Module: crate::block::avx2
// Provides: {"impl_61"}
// Dependencies: {}
impl PartialEq for Block { # [inline] fn eq (& self , other : & Self) -> bool { unsafe { let neq = _mm256_xor_si256 (self . 0 , other . 0) ; _mm256_testz_si256 (neq , neq) == 1 } } }
};
}
