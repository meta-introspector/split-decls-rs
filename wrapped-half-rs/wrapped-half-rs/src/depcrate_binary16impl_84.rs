// Generated macro for impl_84 (impl)
macro_rules! Depcrate_binary16impl_84 {
() => {
// Module: crate::binary16
// Provides: {"impl_84"}
// Dependencies: {}
# [cfg (not (target_arch = "spirv"))] impl Debug for f16 { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { Debug :: fmt (& self . to_f32 () , f) } }
};
}
