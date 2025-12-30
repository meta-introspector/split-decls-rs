// Generated macro for impl_201 (impl)
macro_rules! Depcrate_x86_64_sse2impl_201 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_201"}
// Dependencies: {}
impl < S3 : Copy , S4 : Copy , NI : Copy > u64x2x2 < Machine86 < S3 , S4 , NI > > for u64x2x2_sse2 < S3 , S4 , NI > where u64x2_sse2 < S3 , S4 , NI > : RotateEachWord64 + RotateEachWord32 + BSwap , Machine86 < S3 , S4 , NI > : Machine , u64x2x2_sse2 < S3 , S4 , NI > : MultiLane < [< Machine86 < S3 , S4 , NI > as Machine > :: u64x2 ; 2] > , u64x2x2_sse2 < S3 , S4 , NI > : Vec2 < < Machine86 < S3 , S4 , NI > as Machine > :: u64x2 > , { }
};
}
