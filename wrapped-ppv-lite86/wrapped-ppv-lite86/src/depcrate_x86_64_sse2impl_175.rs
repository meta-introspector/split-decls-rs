// Generated macro for impl_175 (impl)
macro_rules! Depcrate_x86_64_sse2impl_175 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_175"}
// Dependencies: {}
impl < S4 , NI > Words4 for u64x4_sse2 < NoS3 , S4 , NI > { # [inline (always)] fn shuffle2301 (self) -> Self { x2 :: new ([u64x2_sse2 :: new (self . 0 [1] . x) , u64x2_sse2 :: new (self . 0 [0] . x)]) } # [inline (always)] fn shuffle3012 (self) -> Self { unsafe { let a = _mm_srli_si128 (self . 0 [0] . x , 8) ; let b = _mm_slli_si128 (self . 0 [0] . x , 8) ; let c = _mm_srli_si128 (self . 0 [1] . x , 8) ; let d = _mm_slli_si128 (self . 0 [1] . x , 8) ; let da = _mm_or_si128 (d , a) ; let bc = _mm_or_si128 (b , c) ; x2 :: new ([u64x2_sse2 :: new (da) , u64x2_sse2 :: new (bc)]) } } # [inline (always)] fn shuffle1230 (self) -> Self { unsafe { let a = _mm_srli_si128 (self . 0 [0] . x , 8) ; let b = _mm_slli_si128 (self . 0 [0] . x , 8) ; let c = _mm_srli_si128 (self . 0 [1] . x , 8) ; let d = _mm_slli_si128 (self . 0 [1] . x , 8) ; let da = _mm_or_si128 (d , a) ; let bc = _mm_or_si128 (b , c) ; x2 :: new ([u64x2_sse2 :: new (bc) , u64x2_sse2 :: new (da)]) } } }
};
}
