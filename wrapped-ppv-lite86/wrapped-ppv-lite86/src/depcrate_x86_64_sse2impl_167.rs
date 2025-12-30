// Generated macro for impl_167 (impl)
macro_rules! Depcrate_x86_64_sse2impl_167 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_167"}
// Dependencies: {}
impl < NI : Copy > u64x2 < Avx2Machine < NI > > for u64x2_sse2 < YesS3 , YesS4 , NI > where u64x2_sse2 < YesS3 , YesS4 , NI > : RotateEachWord64 + RotateEachWord32 + BSwap + MultiLane < [u64 ; 2] > + Vec2 < u64 > , Machine86 < YesS3 , YesS4 , NI > : Machine , { }
};
}
