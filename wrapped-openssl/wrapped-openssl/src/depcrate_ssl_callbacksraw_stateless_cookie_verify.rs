// Generated macro for raw_stateless_cookie_verify (function)
macro_rules! Depcrate_ssl_callbacksraw_stateless_cookie_verify {
() => {
// Module: crate::ssl::callbacks
// Provides: {"raw_stateless_cookie_verify"}
// Dependencies: {}
# [cfg (ossl111)] pub unsafe extern "C" fn raw_stateless_cookie_verify < F > (ssl : * mut ffi :: SSL , cookie : * const c_uchar , cookie_len : size_t ,) -> c_int where F : Fn (& mut SslRef , & [u8]) -> bool + 'static + Sync + Send , { let ssl = SslRef :: from_ptr_mut (ssl) ; let callback = ssl . ssl_context () . ex_data (SslContext :: cached_ex_index :: < F > ()) . expect ("BUG: stateless cookie verify callback missing") as * const F ; # [allow (clippy :: unnecessary_cast)] let slice = util :: from_raw_parts (cookie as * const c_uchar as * const u8 , cookie_len) ; (* callback) (ssl , slice) as c_int }
};
}
