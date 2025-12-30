// Generated macro for impl_62 (impl)
macro_rules! Depcrate_errorimpl_62 {
() => {
// Module: crate::error
// Provides: {"impl_62"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self . kind { ErrorKind :: None => write ! (f , "no error") , ErrorKind :: Str (ref e) => e . fmt (f) , ErrorKind :: Regex (ref e) => e . fmt (f) , } } }
};
}
