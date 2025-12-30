// Generated macro for impl_5 (impl)
macro_rules! Depcrate_errorimpl_5 {
() => {
// Module: crate::error
// Provides: {"impl_5"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self . 0 { ErrorKind :: Unsupported { actual , expected } => { write ! (f , "unexpected {}, expected {}" , actual , expected) } ErrorKind :: InvalidValue { reason } => { write ! (f , "the value is invalid: {}" , reason) } ErrorKind :: OutsideContainer { method } => { write ! (f , "expected a fragment while buffering {}" , method) } ErrorKind :: NoAlloc { method } => write ! (f , "cannot allocate for {}" , method) , } } }
};
}
