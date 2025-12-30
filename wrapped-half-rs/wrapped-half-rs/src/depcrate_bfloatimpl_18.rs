// Generated macro for impl_18 (impl)
macro_rules! Depcrate_bfloatimpl_18 {
() => {
// Module: crate::bfloat
// Provides: {"impl_18"}
// Dependencies: {}
# [cfg (not (target_arch = "spirv"))] impl FromStr for bf16 { type Err = ParseFloatError ; fn from_str (src : & str) -> Result < bf16 , ParseFloatError > { f32 :: from_str (src) . map (bf16 :: from_f32) } }
};
}
