// Generated macro for raw_cookie_generate (function)
macro_rules! Depcrate_ssl_callbacksraw_cookie_generate {
() => {
// Module: crate::ssl::callbacks
// Provides: {"raw_cookie_generate"}
// Dependencies: {}
# [cfg (not (any (boringssl , awslc)))] pub extern "C" fn raw_cookie_generate < F > (ssl : * mut ffi :: SSL , cookie : * mut c_uchar , cookie_len : * mut c_uint ,) -> c_int where F : Fn (& mut SslRef , & mut [u8]) -> Result < usize , ErrorStack > + 'static + Sync + Send , { unsafe { let ssl = SslRef :: from_ptr_mut (ssl) ; let callback = ssl . ssl_context () . ex_data (SslContext :: cached_ex_index :: < F > ()) . expect ("BUG: cookie generate callback missing") as * const F ; # [allow (clippy :: unnecessary_cast)] let slice = util :: from_raw_parts_mut (cookie as * mut u8 , ffi :: DTLS1_COOKIE_LENGTH as usize - 1) ; match (* callback) (ssl , slice) { Ok (len) => { * cookie_len = len as c_uint ; 1 } Err (e) => { e . put () ; 0 } } } }
};
}
