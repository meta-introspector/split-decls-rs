// Generated macro for impl_146 (impl)
macro_rules! Depcrate_x86_64_sse2impl_146 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_146"}
// Dependencies: {}
impl < S3 , NI > MultiLane < [u32 ; 4] > for u32x4_sse2 < S3 , NoS4 , NI > { # [inline (always)] fn to_lanes (self) -> [u32 ; 4] { unsafe { let x = _mm_cvtsi128_si64 (self . x) as u64 ; let y = _mm_cvtsi128_si64 (_mm_shuffle_epi32 (self . x , 0b11101110)) as u64 ; [x as u32 , (x >> 32) as u32 , y as u32 , (y >> 32) as u32] } } # [inline (always)] fn from_lanes (xs : [u32 ; 4]) -> Self { unsafe { let x = (xs [0] as u64 | ((xs [1] as u64) << 32)) as i64 ; let y = (xs [2] as u64 | ((xs [3] as u64) << 32)) as i64 ; let x = _mm_cvtsi64_si128 (x) ; let y = _mm_slli_si128 (_mm_cvtsi64_si128 (y) , 8) ; Self :: new (_mm_or_si128 (x , y)) } } }
};
}
