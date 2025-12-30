// Generated macro for impl_12 (impl)
macro_rules! Depcrate_errorimpl_12 {
() => {
// Module: crate::error
// Provides: {"impl_12"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . kind { ErrorKind :: None => write ! (f , "no error") , ErrorKind :: Str (ref e) => e . fmt (f) , ErrorKind :: Regex (ref e) => e . fmt (f) , ErrorKind :: Nul (ref e) => e . fmt (f) , } } }
};
}
