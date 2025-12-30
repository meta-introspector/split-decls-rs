// Generated macro for impl_83 (impl)
macro_rules! Depcrate_binary16impl_83 {
() => {
// Module: crate::binary16
// Provides: {"impl_83"}
// Dependencies: {}
# [cfg (not (target_arch = "spirv"))] impl FromStr for f16 { type Err = ParseFloatError ; fn from_str (src : & str) -> Result < f16 , ParseFloatError > { f32 :: from_str (src) . map (f16 :: from_f32) } }
};
}
