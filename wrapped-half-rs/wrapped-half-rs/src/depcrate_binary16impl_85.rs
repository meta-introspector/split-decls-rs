// Generated macro for impl_85 (impl)
macro_rules! Depcrate_binary16impl_85 {
() => {
// Module: crate::binary16
// Provides: {"impl_85"}
// Dependencies: {}
# [cfg (not (target_arch = "spirv"))] impl Display for f16 { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { Display :: fmt (& self . to_f32 () , f) } }
};
}
