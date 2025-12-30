// Generated macro for raw_client_hello (function)
macro_rules! Depcrate_ssl_callbacksraw_client_hello {
() => {
// Module: crate::ssl::callbacks
// Provides: {"raw_client_hello"}
// Dependencies: {}
# [cfg (ossl111)] pub unsafe extern "C" fn raw_client_hello < F > (ssl : * mut ffi :: SSL , al : * mut c_int , arg : * mut c_void ,) -> c_int where F : Fn (& mut SslRef , & mut SslAlert) -> Result < ClientHelloResponse , ErrorStack > + 'static + Sync + Send , { let ssl = SslRef :: from_ptr_mut (ssl) ; let callback = arg as * const F ; let mut alert = SslAlert (* al) ; let r = (* callback) (ssl , & mut alert) ; * al = alert . 0 ; match r { Ok (c) => c . 0 , Err (e) => { e . put () ; ffi :: SSL_CLIENT_HELLO_ERROR } } }
};
}
