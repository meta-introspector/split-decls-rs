// Generated macro for impl_177 (impl)
macro_rules! Depcrate_math_support_bigimpl_177 {
() => {
// Module: crate::math::support::big
// Provides: {"impl_177"}
// Dependencies: {}
impl MinInt for u256 { type OtherSign = i256 ; type Unsigned = u256 ; const SIGNED : bool = false ; const BITS : u32 = 256 ; const ZERO : Self = Self { lo : 0 , hi : 0 } ; const ONE : Self = Self { lo : 1 , hi : 0 } ; const MIN : Self = Self { lo : 0 , hi : 0 } ; const MAX : Self = Self { lo : u128 :: MAX , hi : u128 :: MAX , } ; }
};
}
