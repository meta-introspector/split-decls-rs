// Generated macro for impl_438 (impl)
macro_rules! Depcrate_uri_pathimpl_438 {
() => {
// Module: crate::uri::path
// Provides: {"impl_438"}
// Dependencies: {}
impl fmt :: Display for PathAndQuery { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if ! self . data . is_empty () { match self . data . as_bytes () [0] { b'/' | b'*' => write ! (fmt , "{}" , & self . data [..]) , _ => write ! (fmt , "/{}" , & self . data [..]) , } } else { write ! (fmt , "/") } } }
};
}
