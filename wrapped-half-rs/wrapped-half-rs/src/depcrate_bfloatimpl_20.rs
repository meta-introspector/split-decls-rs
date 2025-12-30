// Generated macro for impl_20 (impl)
macro_rules! Depcrate_bfloatimpl_20 {
() => {
// Module: crate::bfloat
// Provides: {"impl_20"}
// Dependencies: {}
# [cfg (not (target_arch = "spirv"))] impl Display for bf16 { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { Display :: fmt (& self . to_f32 () , f) } }
};
}
