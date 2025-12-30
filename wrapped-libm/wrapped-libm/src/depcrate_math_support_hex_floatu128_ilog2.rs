// Generated macro for u128_ilog2 (function)
macro_rules! Depcrate_math_support_hex_floatu128_ilog2 {
() => {
// Module: crate::math::support::hex_float
// Provides: {"u128_ilog2"}
// Dependencies: {}
# [doc = " `u128::ilog2`"] const fn u128_ilog2 (v : u128) -> u32 { assert ! (v != 0) ; u128 :: BITS - 1 - v . leading_zeros () }
};
}
