// Generated macro for impl_212 (impl)
macro_rules! Depcrate_x86_64_sse2impl_212 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_212"}
// Dependencies: {}
impl < NI : Copy > u64x2x4 < Avx2Machine < NI > > for u64x2x4_sse2 < YesS3 , YesS4 , NI > where u64x2_sse2 < YesS3 , YesS4 , NI > : RotateEachWord64 + RotateEachWord32 + BSwap , Avx2Machine < NI > : Machine , u64x2x4_sse2 < YesS3 , YesS4 , NI > : MultiLane < [< Avx2Machine < NI > as Machine > :: u64x2 ; 4] > , u64x2x4_sse2 < YesS3 , YesS4 , NI > : Vec4 < < Avx2Machine < NI > as Machine > :: u64x2 > , { }
};
}
