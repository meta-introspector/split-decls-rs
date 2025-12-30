// Generated macro for raw_new_session (function)
macro_rules! Depcrate_ssl_callbacksraw_new_session {
() => {
// Module: crate::ssl::callbacks
// Provides: {"raw_new_session"}
// Dependencies: {}
pub unsafe extern "C" fn raw_new_session < F > (ssl : * mut ffi :: SSL , session : * mut ffi :: SSL_SESSION ,) -> c_int where F : Fn (& mut SslRef , SslSession) + 'static + Sync + Send , { let session_ctx_index = try_get_session_ctx_index () . expect ("BUG: session context index initialization failed") ; let ssl = SslRef :: from_ptr_mut (ssl) ; let callback = ssl . ex_data (* session_ctx_index) . expect ("BUG: session context missing") . ex_data (SslContext :: cached_ex_index :: < F > ()) . expect ("BUG: new session callback missing") as * const F ; let session = SslSession :: from_ptr (session) ; (* callback) (ssl , session) ; 1 }
};
}
