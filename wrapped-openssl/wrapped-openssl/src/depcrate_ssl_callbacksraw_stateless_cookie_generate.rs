// Generated macro for raw_stateless_cookie_generate (function)
macro_rules! Depcrate_ssl_callbacksraw_stateless_cookie_generate {
() => {
// Module: crate::ssl::callbacks
// Provides: {"raw_stateless_cookie_generate"}
// Dependencies: {}
# [cfg (ossl111)] pub unsafe extern "C" fn raw_stateless_cookie_generate < F > (ssl : * mut ffi :: SSL , cookie : * mut c_uchar , cookie_len : * mut size_t ,) -> c_int where F : Fn (& mut SslRef , & mut [u8]) -> Result < usize , ErrorStack > + 'static + Sync + Send , { let ssl = SslRef :: from_ptr_mut (ssl) ; let callback = ssl . ssl_context () . ex_data (SslContext :: cached_ex_index :: < F > ()) . expect ("BUG: stateless cookie generate callback missing") as * const F ; # [allow (clippy :: unnecessary_cast)] let slice = util :: from_raw_parts_mut (cookie as * mut u8 , ffi :: SSL_COOKIE_LENGTH as usize) ; match (* callback) (ssl , slice) { Ok (len) => { * cookie_len = len as size_t ; 1 } Err (e) => { e . put () ; 0 } } }
};
}
