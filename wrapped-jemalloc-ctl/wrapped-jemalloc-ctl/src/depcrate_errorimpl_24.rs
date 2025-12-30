// Generated macro for impl_24 (impl)
macro_rules! Depcrate_errorimpl_24 {
() => {
// Module: crate::error
// Provides: {"impl_24"}
// Dependencies: {}
impl fmt :: Debug for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let code = self . 0 . get () as c_int ; match description (code) { Some (m) => write ! (f , "{m}") , None => write ! (f , "Unknown error code: \"{code}\".") , } } }
};
}
