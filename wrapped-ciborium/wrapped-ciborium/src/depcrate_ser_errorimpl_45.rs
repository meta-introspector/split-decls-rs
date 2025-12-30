// Generated macro for impl_45 (impl)
macro_rules! Depcrate_ser_errorimpl_45 {
() => {
// Module: crate::ser::error
// Provides: {"impl_45"}
// Dependencies: {}
impl < T : Debug > Display for Error < T > { # [inline] fn fmt (& self , f : & mut Formatter < '_ >) -> Result { write ! (f , "{self:?}") } }
};
}
