// Generated macro for impl_26 (impl)
macro_rules! Depcrate_bfloatimpl_26 {
() => {
// Module: crate::bfloat
// Provides: {"impl_26"}
// Dependencies: {}
# [cfg (not (target_arch = "spirv"))] impl UpperHex for bf16 { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { write ! (f , "{:X}" , self . 0) } }
};
}
