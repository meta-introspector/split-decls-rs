// Generated macro for impl_586 (impl)
macro_rules! Depcrate_errorimpl_586 {
() => {
// Module: crate::error
// Provides: {"impl_586"}
// Dependencies: {}
impl < F : ErrorFormatter > Debug for Error < F > { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , fmt :: Error > { self . inner . fmt (f) } }
};
}
