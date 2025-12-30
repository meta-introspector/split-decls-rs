// Generated macro for impl_7 (impl)
macro_rules! Depcrate_errorimpl_7 {
() => {
// Module: crate::error
// Provides: {"impl_7"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self . 0 { ErrorKind :: Buffer (ref _e) => { write ! (f , "failed to buffer a value") } ErrorKind :: InvalidValue { reason } => { write ! (f , "the value is invalid: {}" , reason) } # [cfg (not (feature = "alloc"))] ErrorKind :: NoAlloc { method } => write ! (f , "cannot allocate for {}" , method) , } } }
};
}
