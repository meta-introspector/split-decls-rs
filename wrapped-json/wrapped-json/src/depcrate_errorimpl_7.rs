// Generated macro for impl_7 (impl)
macro_rules! Depcrate_errorimpl_7 {
() => {
// Module: crate::error
// Provides: {"impl_7"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self . kind { ErrorKind :: Generic => write ! (f , "an error occurred serializing a value to JSON") , # [cfg (feature = "std")] ErrorKind :: IO (_) => write ! (f , "failed to write JSON") , ErrorKind :: InvalidKey => write ! (f , "attempt to serialize a non-string key") , } } }
};
}
