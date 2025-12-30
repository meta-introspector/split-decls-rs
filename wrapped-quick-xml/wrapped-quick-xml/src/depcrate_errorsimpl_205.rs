// Generated macro for impl_205 (impl)
macro_rules! Depcrate_errorsimpl_205 {
() => {
// Module: crate::errors
// Provides: {"impl_205"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { Self :: Io (e) => write ! (f , "I/O error: {}" , e) , Self :: Syntax (e) => write ! (f , "syntax error: {}" , e) , Self :: IllFormed (e) => write ! (f , "ill-formed document: {}" , e) , Self :: InvalidAttr (e) => write ! (f , "error while parsing attribute: {}" , e) , Self :: Encoding (e) => e . fmt (f) , Self :: Escape (e) => e . fmt (f) , Self :: Namespace (e) => e . fmt (f) , } } }
};
}
