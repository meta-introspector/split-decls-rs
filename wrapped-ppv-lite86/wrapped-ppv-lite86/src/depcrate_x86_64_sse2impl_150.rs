// Generated macro for impl_150 (impl)
macro_rules! Depcrate_x86_64_sse2impl_150 {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_150"}
// Dependencies: {}
impl < S3 , S4 , NI > MultiLane < [u64 ; 4] > for u64x4_sse2 < S3 , S4 , NI > where u64x2_sse2 < S3 , S4 , NI > : MultiLane < [u64 ; 2] > + Copy , { # [inline (always)] fn to_lanes (self) -> [u64 ; 4] { let (a , b) = (self . 0 [0] . to_lanes () , self . 0 [1] . to_lanes ()) ; [a [0] , a [1] , b [0] , b [1]] } # [inline (always)] fn from_lanes (xs : [u64 ; 4]) -> Self { let (a , b) = (u64x2_sse2 :: from_lanes ([xs [0] , xs [1]]) , u64x2_sse2 :: from_lanes ([xs [2] , xs [3]]) ,) ; x2 :: new ([a , b]) } }
};
}
