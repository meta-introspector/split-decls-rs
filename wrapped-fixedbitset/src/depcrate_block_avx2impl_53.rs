// Generated macro for impl_53 (impl)
macro_rules! Depcrate_block_avx2impl_53 {
() => {
// Module: crate::block::avx2
// Provides: {"impl_53"}
// Dependencies: {}
impl Block { # [inline] pub fn is_empty (self) -> bool { unsafe { _mm256_testz_si256 (self . 0 , self . 0) == 1 } } # [inline] pub fn andnot (self , other : Self) -> Self { Self (unsafe { _mm256_andnot_si256 (other . 0 , self . 0) }) } }
};
}
