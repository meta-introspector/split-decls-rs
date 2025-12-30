// Generated macro for impl_174 (impl)
macro_rules! Depcrate_x86_64_sse2impl_174 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_174"}
// Dependencies: {}
impl < S4 , NI > Words4 for u64x4_sse2 < YesS3 , S4 , NI > { # [inline (always)] fn shuffle2301 (self) -> Self { x2 :: new ([u64x2_sse2 :: new (self . 0 [1] . x) , u64x2_sse2 :: new (self . 0 [0] . x)]) } # [inline (always)] fn shuffle3012 (self) -> Self { unsafe { x2 :: new ([u64x2_sse2 :: new (_mm_alignr_epi8 (self . 0 [1] . x , self . 0 [0] . x , 8)) , u64x2_sse2 :: new (_mm_alignr_epi8 (self . 0 [0] . x , self . 0 [1] . x , 8)) ,]) } } # [inline (always)] fn shuffle1230 (self) -> Self { unsafe { x2 :: new ([u64x2_sse2 :: new (_mm_alignr_epi8 (self . 0 [0] . x , self . 0 [1] . x , 8)) , u64x2_sse2 :: new (_mm_alignr_epi8 (self . 0 [1] . x , self . 0 [0] . x , 8)) ,]) } } }
};
}
