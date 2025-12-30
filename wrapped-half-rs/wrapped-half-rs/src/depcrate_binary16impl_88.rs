// Generated macro for impl_88 (impl)
macro_rules! Depcrate_binary16impl_88 {
() => {
// Module: crate::binary16
// Provides: {"impl_88"}
// Dependencies: {}
# [cfg (not (target_arch = "spirv"))] impl Binary for f16 { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { write ! (f , "{:b}" , self . 0) } }
};
}
