// Generated macro for raw_sni (function)
macro_rules! Depcrate_ssl_callbacksraw_sni {
() => {
// Module: crate::ssl::callbacks
// Provides: {"raw_sni"}
// Dependencies: {}
pub extern "C" fn raw_sni < F > (ssl : * mut ffi :: SSL , al : * mut c_int , arg : * mut c_void) -> c_int where F : Fn (& mut SslRef , & mut SslAlert) -> Result < () , SniError > + 'static + Sync + Send , { unsafe { let ssl = SslRef :: from_ptr_mut (ssl) ; let callback = arg as * const F ; let mut alert = SslAlert (* al) ; let r = (* callback) (ssl , & mut alert) ; * al = alert . 0 ; match r { Ok (()) => ffi :: SSL_TLSEXT_ERR_OK , Err (e) => e . 0 , } } }
};
}
