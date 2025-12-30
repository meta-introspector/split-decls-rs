// Generated macro for impl_25 (impl)
macro_rules! Depcrate_bfloatimpl_25 {
() => {
// Module: crate::bfloat
// Provides: {"impl_25"}
// Dependencies: {}
# [cfg (not (target_arch = "spirv"))] impl LowerHex for bf16 { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { write ! (f , "{:x}" , self . 0) } }
};
}
