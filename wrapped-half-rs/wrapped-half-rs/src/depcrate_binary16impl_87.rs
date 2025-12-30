// Generated macro for impl_87 (impl)
macro_rules! Depcrate_binary16impl_87 {
() => {
// Module: crate::binary16
// Provides: {"impl_87"}
// Dependencies: {}
# [cfg (not (target_arch = "spirv"))] impl UpperExp for f16 { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { write ! (f , "{:E}" , self . to_f32 ()) } }
};
}
