// Generated macro for impl_86 (impl)
macro_rules! Depcrate_binary16impl_86 {
() => {
// Module: crate::binary16
// Provides: {"impl_86"}
// Dependencies: {}
# [cfg (not (target_arch = "spirv"))] impl LowerExp for f16 { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { write ! (f , "{:e}" , self . to_f32 ()) } }
};
}
