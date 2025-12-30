// Generated macro for impl_314 (impl)
macro_rules! Depcrate_errorimpl_314 {
() => {
// Module: crate::error
// Provides: {"impl_314"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . kind) ? ; if ! self . locations . is_empty () { write ! (f , " at {}" , self . locations . join ("/")) ? ; } Ok (()) } }
};
}
