// Generated macro for impl_11 (impl)
macro_rules! Depcrate_de_errorimpl_11 {
() => {
// Module: crate::de::error
// Provides: {"impl_11"}
// Dependencies: {}
impl < T : Debug > Display for Error < T > { # [inline] fn fmt (& self , f : & mut Formatter < '_ >) -> Result { write ! (f , "{self:?}") } }
};
}
