// Generated macro for impl_185 (impl)
macro_rules! Depcrate_math_support_bigimpl_185 {
() => {
// Module: crate::math::support::big
// Provides: {"impl_185"}
// Dependencies: {}
impl HInt for i128 { type D = i256 ; fn widen (self) -> Self :: D { i256 { lo : self as u128 , hi : if self < 0 { - 1 } else { 0 } , } } fn zero_widen (self) -> Self :: D { self . unsigned () . zero_widen () . signed () } fn zero_widen_mul (self , rhs : Self) -> Self :: D { self . unsigned () . zero_widen_mul (rhs . unsigned ()) . signed () } fn widen_mul (self , _rhs : Self) -> Self :: D { unimplemented ! ("signed i128 widening multiply is not used") } fn widen_hi (self) -> Self :: D { i256 { lo : 0 , hi : self } } }
};
}
