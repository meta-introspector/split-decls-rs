// Generated macro for impl_157 (impl)
macro_rules! Depcrate_errorsimpl_157 {
() => {
// Module: crate::errors
// Provides: {"impl_157"}
// Dependencies: {}
impl fmt :: Display for ErrorKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { ErrorKind :: Help => write ! (f , "HELP") , ErrorKind :: Error => write ! (f , "ERROR") , ErrorKind :: Note => write ! (f , "NOTE") , ErrorKind :: Suggestion => write ! (f , "SUGGESTION") , ErrorKind :: Warning => write ! (f , "WARN") , ErrorKind :: Raw => write ! (f , "RAW") , ErrorKind :: Unknown => write ! (f , "UNKNOWN") , } } }
};
}
