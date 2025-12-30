// Generated macro for impl_8 (impl)
macro_rules! Depcrate_errorimpl_8 {
() => {
// Module: crate::error
// Provides: {"impl_8"}
// Dependencies: {}
impl std :: fmt :: Display for Error { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self . kind { ErrorKind :: Regex (ref s) => write ! (f , "{}" , s) , } } }
};
}
