// Generated macro for f32_from_bits (function)
macro_rules! Depcrate_math_support_float_traitsf32_from_bits {
() => {
// Module: crate::math::support::float_traits
// Provides: {"f32_from_bits"}
// Dependencies: {}
# [doc = " `f32::from_bits`"] # [allow (unnecessary_transmutes)] pub const fn f32_from_bits (bits : u32) -> f32 { unsafe { mem :: transmute :: < u32 , f32 > (bits) } }
};
}
