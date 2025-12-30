// Generated macro for impl_49 (impl)
macro_rules! Depcrate_errorimpl_49 {
() => {
// Module: crate::error
// Provides: {"impl_49"}
// Dependencies: {}
impl Display for Error { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: Message { ref msg , .. } => write ! (formatter , "{}" , msg) , } } }
};
}
