// Generated macro for impl_163 (impl)
macro_rules! Depcrate_x86_64_sse2impl_163 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_163"}
// Dependencies: {}
impl < S3 : Copy , S4 : Copy , NI : Copy > u32x4 < Machine86 < S3 , S4 , NI > > for u32x4_sse2 < S3 , S4 , NI > where u32x4_sse2 < S3 , S4 , NI > : RotateEachWord32 + BSwap + MultiLane < [u32 ; 4] > + Vec4 < u32 > , Machine86 < S3 , S4 , NI > : Machine , { }
};
}
