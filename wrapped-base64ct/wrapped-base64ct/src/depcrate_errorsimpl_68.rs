// Generated macro for impl_68 (impl)
macro_rules! Depcrate_errorsimpl_68 {
() => {
// Module: crate::errors
// Provides: {"impl_68"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { let s = match self { Self :: InvalidEncoding => INVALID_ENCODING_MSG , Self :: InvalidLength => INVALID_LENGTH_MSG , } ; f . write_str (s) } }
};
}
