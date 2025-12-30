// Generated macro for macro_109 (macro)
macro_rules! Depcrate_int_specialized_div_remmacro_109 {
() => {
// Module: crate::int::specialized_div_rem
// Provides: {"macro_109"}
// Dependencies: {}
# [cfg (all (not (all (not (feature = "no-asm") , target_arch = "x86")) , not (target_pointer_width = "64")))] impl_delegate ! (u64_div_rem , zero_div_fn , u32_normalization_shift , u32_by_u32_div_rem , 16 , u16 , u32 , u64 , i64) ;
};
}
