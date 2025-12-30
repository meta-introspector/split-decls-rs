// Generated macro for impl_435 (impl)
macro_rules! Depcrate_errorimpl_435 {
() => {
// Module: crate::error
// Provides: {"impl_435"}
// Dependencies: {}
impl fmt :: Display for Error { # [allow (unused_unsafe)] fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (fmt , "error:{:08X}" , self . code ()) ? ; match self . library () { Some (l) => write ! (fmt , ":{}" , l) ? , None => write ! (fmt , ":lib({})" , self . library_code ()) ? , } match self . function () { Some (f) => write ! (fmt , ":{}" , f) ? , None => write ! (fmt , ":func({})" , unsafe { ffi :: ERR_GET_FUNC (self . code ()) }) ? , } match self . reason () { Some (r) => write ! (fmt , ":{}" , r) ? , None => write ! (fmt , ":reason({})" , self . reason_code ()) ? , } write ! (fmt , ":{}:{}:{}" , self . file () , self . line () , self . data () . unwrap_or ("")) } }
};
}
