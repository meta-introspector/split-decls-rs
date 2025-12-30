// Generated macro for raw_get_session (function)
macro_rules! Depcrate_ssl_callbacksraw_get_session {
() => {
// Module: crate::ssl::callbacks
// Provides: {"raw_get_session"}
// Dependencies: {}
pub unsafe extern "C" fn raw_get_session < F > (ssl : * mut ffi :: SSL , data : DataPtr , len : c_int , copy : * mut c_int ,) -> * mut ffi :: SSL_SESSION where F : Fn (& mut SslRef , & [u8]) -> Option < SslSession > + 'static + Sync + Send , { let session_ctx_index = try_get_session_ctx_index () . expect ("BUG: session context index initialization failed") ; let ssl = SslRef :: from_ptr_mut (ssl) ; let callback = ssl . ex_data (* session_ctx_index) . expect ("BUG: session context missing") . ex_data (SslContext :: cached_ex_index :: < F > ()) . expect ("BUG: get session callback missing") as * const F ; # [allow (clippy :: unnecessary_cast)] let data = util :: from_raw_parts (data as * const u8 , len as usize) ; match (* callback) (ssl , data) { Some (session) => { let p = session . as_ptr () ; mem :: forget (session) ; * copy = 0 ; p } None => ptr :: null_mut () , } }
};
}
