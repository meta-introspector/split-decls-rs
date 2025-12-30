// Generated macro for impl_206 (impl)
macro_rules! Depcrate_x86_64_sse2impl_206 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_206"}
// Dependencies: {}
impl < NI : Copy > u64x4 < Avx2Machine < NI > > for u64x4_sse2 < YesS3 , YesS4 , NI > where u64x2_sse2 < YesS3 , YesS4 , NI > : RotateEachWord64 + RotateEachWord32 + BSwap , Avx2Machine < NI > : Machine , u64x4_sse2 < YesS3 , YesS4 , NI > : MultiLane < [u64 ; 4] > + Vec4 < u64 > + Words4 , { }
};
}
