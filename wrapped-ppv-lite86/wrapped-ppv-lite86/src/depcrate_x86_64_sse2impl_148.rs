// Generated macro for impl_148 (impl)
macro_rules! Depcrate_x86_64_sse2impl_148 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_148"}
// Dependencies: {}
impl < S3 , NI > MultiLane < [u64 ; 2] > for u64x2_sse2 < S3 , NoS4 , NI > { # [inline (always)] fn to_lanes (self) -> [u64 ; 2] { unsafe { [_mm_cvtsi128_si64 (self . x) as u64 , _mm_cvtsi128_si64 (_mm_srli_si128 (self . x , 8)) as u64 ,] } } # [inline (always)] fn from_lanes (xs : [u64 ; 2]) -> Self { unsafe { let x = _mm_cvtsi64_si128 (xs [0] as i64) ; let y = _mm_slli_si128 (_mm_cvtsi64_si128 (xs [1] as i64) , 8) ; Self :: new (_mm_or_si128 (x , y)) } } }
};
}
