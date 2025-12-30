// Generated macro for impl_280 (impl)
macro_rules! Depcrate_memimpl_280 {
() => {
// Module: crate::mem
// Provides: {"impl_280"}
// Dependencies: {}
impl fmt :: Display for DecompressError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let msg = match & self . 0 { DecompressErrorInner :: General { msg } => msg . get () , DecompressErrorInner :: NeedsDictionary { .. } => Some ("requires a dictionary") , } ; match msg { Some (msg) => write ! (f , "deflate decompression error: {msg}") , None => write ! (f , "deflate decompression error") , } } }
};
}
