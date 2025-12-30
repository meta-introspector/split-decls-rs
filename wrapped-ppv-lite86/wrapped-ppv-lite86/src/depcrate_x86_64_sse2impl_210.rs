// Generated macro for impl_210 (impl)
macro_rules! Depcrate_x86_64_sse2impl_210 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_210"}
// Dependencies: {}
impl < S3 : Copy , S4 : Copy , NI : Copy > u64x2x4 < Machine86 < S3 , S4 , NI > > for u64x2x4_sse2 < S3 , S4 , NI > where u64x2_sse2 < S3 , S4 , NI > : RotateEachWord64 + RotateEachWord32 + BSwap , Machine86 < S3 , S4 , NI > : Machine , u64x2x4_sse2 < S3 , S4 , NI > : MultiLane < [< Machine86 < S3 , S4 , NI > as Machine > :: u64x2 ; 4] > , u64x2x4_sse2 < S3 , S4 , NI > : Vec4 < < Machine86 < S3 , S4 , NI > as Machine > :: u64x2 > , { }
};
}
