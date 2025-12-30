// Generated macro for raw_remove_session (function)
macro_rules! Depcrate_ssl_callbacksraw_remove_session {
() => {
// Module: crate::ssl::callbacks
// Provides: {"raw_remove_session"}
// Dependencies: {}
pub unsafe extern "C" fn raw_remove_session < F > (ctx : * mut ffi :: SSL_CTX , session : * mut ffi :: SSL_SESSION ,) where F : Fn (& SslContextRef , & SslSessionRef) + 'static + Sync + Send , { let ctx = SslContextRef :: from_ptr (ctx) ; let callback = ctx . ex_data (SslContext :: cached_ex_index :: < F > ()) . expect ("BUG: remove session callback missing") ; let session = SslSessionRef :: from_ptr (session) ; callback (ctx , session) }
};
}
