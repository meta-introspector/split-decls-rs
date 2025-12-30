// Generated macro for impl_24 (impl)
macro_rules! Depcrate_bfloatimpl_24 {
() => {
// Module: crate::bfloat
// Provides: {"impl_24"}
// Dependencies: {}
# [cfg (not (target_arch = "spirv"))] impl Octal for bf16 { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { write ! (f , "{:o}" , self . 0) } }
};
}
