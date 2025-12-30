// Generated macro for impl_22 (impl)
macro_rules! Depcrate_errorimpl_22 {
() => {
// Module: crate::error
// Provides: {"impl_22"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: InvalidEncoding => f . write_str ("invalid Base32 encoding") , Error :: InvalidLength => f . write_str ("invalid Base32 length") , } } }
};
}
