// Generated macro for impl_28 (impl)
macro_rules! Depcrate_errorimpl_28 {
() => {
// Module: crate::error
// Provides: {"impl_28"}
// Dependencies: {}
impl Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: Overflow => write ! (f , "Output buffer too small or calculation overflow") , Error :: InvalidInput => write ! (f , "Invalid input for the given encoding") , } } }
};
}
