// Generated macro for f32_to_bits (function)
macro_rules! Depcrate_math_support_float_traitsf32_to_bits {
() => {
// Module: crate::math::support::float_traits
// Provides: {"f32_to_bits"}
// Dependencies: {}
# [doc = " `f32::to_bits`"] # [allow (dead_code)] # [allow (unnecessary_transmutes)] pub const fn f32_to_bits (x : f32) -> u32 { unsafe { mem :: transmute :: < f32 , u32 > (x) } }
};
}
