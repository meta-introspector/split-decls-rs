// Generated macro for impl_178 (impl)
macro_rules! Depcrate_math_support_bigimpl_178 {
() => {
// Module: crate::math::support::big
// Provides: {"impl_178"}
// Dependencies: {}
impl i256 { # [doc = " Reinterpret as an unsigned integer"] # [cfg (any (test , feature = "unstable-public-internals"))] pub fn unsigned (self) -> u256 { u256 { lo : self . lo , hi : self . hi as u128 , } } }
};
}
