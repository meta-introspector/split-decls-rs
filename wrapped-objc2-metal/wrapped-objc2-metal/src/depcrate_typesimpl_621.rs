// Generated macro for impl_621 (impl)
macro_rules! Depcrate_typesimpl_621 {
() => {
// Module: crate::types
// Provides: {"impl_621"}
// Dependencies: {}
unsafe impl Encode for MTLResourceID { # [allow (unexpected_cfgs)] const ENCODING : Encoding = Encoding :: Struct ("MTLResourceID" , if cfg ! (target_env = "sim") { & [Encoding :: Union ("?" , & [< u64 > :: ENCODING , < u64 > :: ENCODING])] } else { & [< u64 > :: ENCODING] } ,) ; }
};
}
