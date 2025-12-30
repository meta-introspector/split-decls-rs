// Generated macro for impl_53 (impl)
macro_rules! Depcrate_argsimpl_53 {
() => {
// Module: crate::args
// Provides: {"impl_53"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: Utf8Error (path) => write ! (fmt , "UTF-8 error in {path}") , Error :: IOError (path , err) => write ! (fmt , "IO error: {path}: {err}") , Error :: ShellParseError (path) => write ! (fmt , "invalid shell-style arguments in {path}") , } } }
};
}
