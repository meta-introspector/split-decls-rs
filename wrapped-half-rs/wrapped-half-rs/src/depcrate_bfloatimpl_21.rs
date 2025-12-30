// Generated macro for impl_21 (impl)
macro_rules! Depcrate_bfloatimpl_21 {
() => {
// Module: crate::bfloat
// Provides: {"impl_21"}
// Dependencies: {}
# [cfg (not (target_arch = "spirv"))] impl LowerExp for bf16 { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { write ! (f , "{:e}" , self . to_f32 ()) } }
};
}
