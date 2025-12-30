// Generated macro for f64_from_bits (function)
macro_rules! Depcrate_math_support_float_traitsf64_from_bits {
() => {
// Module: crate::math::support::float_traits
// Provides: {"f64_from_bits"}
// Dependencies: {}
# [doc = " `f64::from_bits`"] # [allow (unnecessary_transmutes)] pub const fn f64_from_bits (bits : u64) -> f64 { unsafe { mem :: transmute :: < u64 , f64 > (bits) } }
};
}
