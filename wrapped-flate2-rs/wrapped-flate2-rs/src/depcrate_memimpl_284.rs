// Generated macro for impl_284 (impl)
macro_rules! Depcrate_memimpl_284 {
() => {
// Module: crate::mem
// Provides: {"impl_284"}
// Dependencies: {}
impl fmt :: Display for CompressError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self . msg . get () { Some (msg) => write ! (f , "deflate compression error: {msg}") , None => write ! (f , "deflate compression error") , } } }
};
}
