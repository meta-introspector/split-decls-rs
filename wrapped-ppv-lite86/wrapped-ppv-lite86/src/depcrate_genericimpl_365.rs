// Generated macro for impl_365 (impl)
macro_rules! Depcrate_genericimpl_365 {
() => {
// Module: crate::generic
// Provides: {"impl_365"}
// Dependencies: {}
impl MultiLane < [u64 ; 4] > for u64x4_generic { # [inline (always)] fn to_lanes (self) -> [u64 ; 4] { let (a , b) = (self . 0 [0] . to_lanes () , self . 0 [1] . to_lanes ()) ; [a [0] , a [1] , b [0] , b [1]] } # [inline (always)] fn from_lanes (xs : [u64 ; 4]) -> Self { let (a , b) = (u64x2_generic :: from_lanes ([xs [0] , xs [1]]) , u64x2_generic :: from_lanes ([xs [2] , xs [3]]) ,) ; x2 :: new ([a , b]) } }
};
}
