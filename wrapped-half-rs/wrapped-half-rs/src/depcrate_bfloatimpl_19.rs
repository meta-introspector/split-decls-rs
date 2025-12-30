// Generated macro for impl_19 (impl)
macro_rules! Depcrate_bfloatimpl_19 {
() => {
// Module: crate::bfloat
// Provides: {"impl_19"}
// Dependencies: {}
# [cfg (not (target_arch = "spirv"))] impl Debug for bf16 { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { Debug :: fmt (& self . to_f32 () , f) } }
};
}
