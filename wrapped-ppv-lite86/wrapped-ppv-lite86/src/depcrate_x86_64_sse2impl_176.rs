// Generated macro for impl_176 (impl)
macro_rules! Depcrate_x86_64_sse2impl_176 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_176"}
// Dependencies: {}
impl < S3 , S4 , NI > UnsafeFrom < [u64 ; 2] > for u64x2_sse2 < S3 , S4 , NI > { # [inline (always)] unsafe fn unsafe_from (xs : [u64 ; 2]) -> Self { Self :: new (_mm_set_epi64x (xs [1] as i64 , xs [0] as i64)) } }
};
}
