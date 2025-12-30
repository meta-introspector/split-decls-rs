// Generated macro for impl_38 (impl)
macro_rules! Depcrate_errorimpl_38 {
() => {
// Module: crate::error
// Provides: {"impl_38"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: InvalidEncoding => f . write_str ("invalid Base16 encoding") , Error :: InvalidLength => f . write_str ("invalid Base16 length") , } } }
};
}
