// Generated macro for raw_tlsext_status (function)
macro_rules! Depcrate_ssl_callbacksraw_tlsext_status {
() => {
// Module: crate::ssl::callbacks
// Provides: {"raw_tlsext_status"}
// Dependencies: {}
pub unsafe extern "C" fn raw_tlsext_status < F > (ssl : * mut ffi :: SSL , _ : * mut c_void) -> c_int where F : Fn (& mut SslRef) -> Result < bool , ErrorStack > + 'static + Sync + Send , { let ssl = SslRef :: from_ptr_mut (ssl) ; let callback = ssl . ssl_context () . ex_data (SslContext :: cached_ex_index :: < F > ()) . expect ("BUG: ocsp callback missing") as * const F ; let ret = (* callback) (ssl) ; if ssl . is_server () { match ret { Ok (true) => ffi :: SSL_TLSEXT_ERR_OK , Ok (false) => ffi :: SSL_TLSEXT_ERR_NOACK , Err (e) => { e . put () ; ffi :: SSL_TLSEXT_ERR_ALERT_FATAL } } } else { match ret { Ok (true) => 1 , Ok (false) => 0 , Err (e) => { e . put () ; - 1 } } } }
};
}
