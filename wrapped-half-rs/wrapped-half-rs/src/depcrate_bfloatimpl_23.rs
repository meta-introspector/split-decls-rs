// Generated macro for impl_23 (impl)
macro_rules! Depcrate_bfloatimpl_23 {
() => {
// Module: crate::bfloat
// Provides: {"impl_23"}
// Dependencies: {}
# [cfg (not (target_arch = "spirv"))] impl Binary for bf16 { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { write ! (f , "{:b}" , self . 0) } }
};
}
