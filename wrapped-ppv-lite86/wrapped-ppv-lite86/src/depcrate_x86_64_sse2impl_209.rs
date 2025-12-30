// Generated macro for impl_209 (impl)
macro_rules! Depcrate_x86_64_sse2impl_209 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_209"}
// Dependencies: {}
impl < S3 : Copy , S4 : Copy , NI : Copy > u32x4x4 < Machine86 < S3 , S4 , NI > > for u32x4x4_sse2 < S3 , S4 , NI > where u32x4_sse2 < S3 , S4 , NI > : RotateEachWord32 + BSwap , Machine86 < S3 , S4 , NI > : Machine , u32x4x4_sse2 < S3 , S4 , NI > : MultiLane < [< Machine86 < S3 , S4 , NI > as Machine > :: u32x4 ; 4] > , u32x4x4_sse2 < S3 , S4 , NI > : Vec4 < < Machine86 < S3 , S4 , NI > as Machine > :: u32x4 > , u32x4x4_sse2 < S3 , S4 , NI > : Vec4Ext < < Machine86 < S3 , S4 , NI > as Machine > :: u32x4 > , u32x4x4_sse2 < S3 , S4 , NI > : Vector < [u32 ; 16] > , { }
};
}
