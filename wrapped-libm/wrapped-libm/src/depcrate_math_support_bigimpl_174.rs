// Generated macro for impl_174 (impl)
macro_rules! Depcrate_math_support_bigimpl_174 {
() => {
// Module: crate::math::support::big
// Provides: {"impl_174"}
// Dependencies: {}
impl u256 { # [cfg (any (test , feature = "unstable-public-internals"))] pub const MAX : Self = Self { lo : u128 :: MAX , hi : u128 :: MAX , } ; # [doc = " Reinterpret as a signed integer"] pub fn signed (self) -> i256 { i256 { lo : self . lo , hi : self . hi as i128 , } } }
};
}
