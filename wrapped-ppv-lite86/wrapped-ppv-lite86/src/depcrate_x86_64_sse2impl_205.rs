// Generated macro for impl_205 (impl)
macro_rules! Depcrate_x86_64_sse2impl_205 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_205"}
// Dependencies: {}
impl < NI : Copy > u64x2x2 < Avx2Machine < NI > > for u64x2x2_sse2 < YesS3 , YesS4 , NI > where u64x2_sse2 < YesS3 , YesS4 , NI > : RotateEachWord64 + RotateEachWord32 + BSwap , Avx2Machine < NI > : Machine , u64x2x2_sse2 < YesS3 , YesS4 , NI > : MultiLane < [< Avx2Machine < NI > as Machine > :: u64x2 ; 2] > , u64x2x2_sse2 < YesS3 , YesS4 , NI > : Vec2 < < Avx2Machine < NI > as Machine > :: u64x2 > , { }
};
}
