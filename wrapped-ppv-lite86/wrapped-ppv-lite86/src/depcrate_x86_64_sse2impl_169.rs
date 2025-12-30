// Generated macro for impl_169 (impl)
macro_rules! Depcrate_x86_64_sse2impl_169 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_169"}
// Dependencies: {}
impl < S3 , S4 , NI > UnsafeFrom < [u32 ; 4] > for u32x4_sse2 < S3 , S4 , NI > { # [inline (always)] unsafe fn unsafe_from (xs : [u32 ; 4]) -> Self { Self :: new (_mm_set_epi32 (xs [3] as i32 , xs [2] as i32 , xs [1] as i32 , xs [0] as i32 ,)) } }
};
}
