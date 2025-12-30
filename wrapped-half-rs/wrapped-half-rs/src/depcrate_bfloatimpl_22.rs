// Generated macro for impl_22 (impl)
macro_rules! Depcrate_bfloatimpl_22 {
() => {
// Module: crate::bfloat
// Provides: {"impl_22"}
// Dependencies: {}
# [cfg (not (target_arch = "spirv"))] impl UpperExp for bf16 { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { write ! (f , "{:E}" , self . to_f32 ()) } }
};
}
