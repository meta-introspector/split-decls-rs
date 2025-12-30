// Generated macro for impl_585 (impl)
macro_rules! Depcrate_errorimpl_585 {
() => {
// Module: crate::error
// Provides: {"impl_585"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . message) ? ; match self . class () { ErrorClass :: None => { } other => write ! (f , "; class={:?} ({})" , other , self . klass) ? , } match self . code () { ErrorCode :: GenericError => { } other => write ! (f , "; code={:?} ({})" , other , self . code) ? , } Ok (()) } }
};
}
