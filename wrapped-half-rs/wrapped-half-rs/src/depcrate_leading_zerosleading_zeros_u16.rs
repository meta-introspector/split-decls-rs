// Generated macro for leading_zeros_u16 (function)
macro_rules! Depcrate_leading_zerosleading_zeros_u16 {
() => {
// Module: crate::leading_zeros
// Provides: {"leading_zeros_u16"}
// Dependencies: {}
# [cfg (all (target_arch = "spirv" , not (all (target_feature = "IntegerFunctions2INTEL" , target_feature = "SPV_INTEL_shader_integer_functions2"))))] # [inline] pub (crate) const fn leading_zeros_u16 (x : u16) -> u32 { leading_zeros_u16_fallback (x) }
};
}
