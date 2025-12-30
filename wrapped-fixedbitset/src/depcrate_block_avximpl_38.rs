// Generated macro for impl_38 (impl)
macro_rules! Depcrate_block_avximpl_38 {
() => {
// Module: crate::block::avx
// Provides: {"impl_38"}
// Dependencies: {}
impl Block { # [inline] pub fn is_empty (self) -> bool { unsafe { let value = _mm256_castpd_si256 (self . 0) ; _mm256_testz_si256 (value , value) == 1 } } # [inline] pub fn andnot (self , other : Self) -> Self { unsafe { Self (_mm256_andnot_pd (other . 0 , self . 0)) } } }
};
}
