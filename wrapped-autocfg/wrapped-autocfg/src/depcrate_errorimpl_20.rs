// Generated macro for impl_20 (impl)
macro_rules! Depcrate_errorimpl_20 {
() => {
// Module: crate::error
// Provides: {"impl_20"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> Result < () , fmt :: Error > { match self . kind { ErrorKind :: Io (ref e) => e . fmt (f) , ErrorKind :: Num (ref e) => e . fmt (f) , ErrorKind :: Utf8 (ref e) => e . fmt (f) , ErrorKind :: Process (ref status) => { write ! (f , "process exited unsuccessfully: {}" , status) } ErrorKind :: Other (s) => s . fmt (f) , } } }
};
}
