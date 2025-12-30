// Generated macro for impl_164 (impl)
macro_rules! Depcrate_x86_64_sse2impl_164 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_164"}
// Dependencies: {}
impl < S3 : Copy , S4 : Copy , NI : Copy > u64x2 < Machine86 < S3 , S4 , NI > > for u64x2_sse2 < S3 , S4 , NI > where u64x2_sse2 < S3 , S4 , NI > : RotateEachWord64 + RotateEachWord32 + BSwap + MultiLane < [u64 ; 2] > + Vec2 < u64 > , Machine86 < S3 , S4 , NI > : Machine , { }
};
}
