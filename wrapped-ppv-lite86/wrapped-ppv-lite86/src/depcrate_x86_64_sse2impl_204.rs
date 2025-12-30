// Generated macro for impl_204 (impl)
macro_rules! Depcrate_x86_64_sse2impl_204 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_204"}
// Dependencies: {}
impl < NI : Copy > u32x4x2 < Avx2Machine < NI > > for u32x4x2_sse2 < YesS3 , YesS4 , NI > where u32x4_sse2 < YesS3 , YesS4 , NI > : RotateEachWord32 + BSwap , Avx2Machine < NI > : Machine , u32x4x2_sse2 < YesS3 , YesS4 , NI > : MultiLane < [< Avx2Machine < NI > as Machine > :: u32x4 ; 2] > , u32x4x2_sse2 < YesS3 , YesS4 , NI > : Vec2 < < Avx2Machine < NI > as Machine > :: u32x4 > , { }
};
}
