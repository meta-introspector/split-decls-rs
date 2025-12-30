// Generated macro for leading_zeros_u16_fallback (function)
macro_rules! Depcrate_leading_zerosleading_zeros_u16_fallback {
() => {
// Module: crate::leading_zeros
// Provides: {"leading_zeros_u16_fallback"}
// Dependencies: {}
# [cfg (any (test , all (target_arch = "spirv" , not (all (target_feature = "IntegerFunctions2INTEL" , target_feature = "SPV_INTEL_shader_integer_functions2")))))] # [inline] const fn leading_zeros_u16_fallback (mut x : u16) -> u32 { use crunchy :: unroll ; let mut c = 0 ; let msb = 1 << 15 ; unroll ! { for i in 0 .. 16 { if x & msb == 0 { c += 1 ; } else { return c ; } # [allow (unused_assignments)] if i < 15 { x <<= 1 ; } } } c }
};
}
