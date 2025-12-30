// Generated macro for raw_keylog (function)
macro_rules! Depcrate_ssl_callbacksraw_keylog {
() => {
// Module: crate::ssl::callbacks
// Provides: {"raw_keylog"}
// Dependencies: {}
# [cfg (any (ossl111 , boringssl , awslc))] pub unsafe extern "C" fn raw_keylog < F > (ssl : * const ffi :: SSL , line : * const c_char) where F : Fn (& SslRef , & str) + 'static + Sync + Send , { let ssl = SslRef :: from_const_ptr (ssl) ; let callback = ssl . ssl_context () . ex_data (SslContext :: cached_ex_index :: < F > ()) . expect ("BUG: get session callback missing") ; let line = CStr :: from_ptr (line) . to_bytes () ; let line = str :: from_utf8_unchecked (line) ; callback (ssl , line) ; }
};
}
