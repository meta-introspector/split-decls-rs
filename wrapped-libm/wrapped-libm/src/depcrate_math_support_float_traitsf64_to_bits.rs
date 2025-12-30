// Generated macro for f64_to_bits (function)
macro_rules! Depcrate_math_support_float_traitsf64_to_bits {
() => {
// Module: crate::math::support::float_traits
// Provides: {"f64_to_bits"}
// Dependencies: {}
# [doc = " `f64::to_bits`"] # [allow (dead_code)] # [allow (unnecessary_transmutes)] pub const fn f64_to_bits (x : f64) -> u64 { unsafe { mem :: transmute :: < f64 , u64 > (x) } }
};
}
