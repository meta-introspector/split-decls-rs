// Generated macro for raw_alpn_select (function)
macro_rules! Depcrate_ssl_callbacksraw_alpn_select {
() => {
// Module: crate::ssl::callbacks
// Provides: {"raw_alpn_select"}
// Dependencies: {}
pub extern "C" fn raw_alpn_select < F > (ssl : * mut ffi :: SSL , out : * mut * const c_uchar , outlen : * mut c_uchar , inbuf : * const c_uchar , inlen : c_uint , _arg : * mut c_void ,) -> c_int where F : for < 'a > Fn (& mut SslRef , & 'a [u8]) -> Result < & 'a [u8] , AlpnError > + 'static + Sync + Send , { unsafe { let ssl = SslRef :: from_ptr_mut (ssl) ; let callback = ssl . ssl_context () . ex_data (SslContext :: cached_ex_index :: < F > ()) . expect ("BUG: alpn callback missing") as * const F ; # [allow (clippy :: unnecessary_cast)] let protos = util :: from_raw_parts (inbuf as * const u8 , inlen as usize) ; match (* callback) (ssl , protos) { Ok (proto) => { * out = proto . as_ptr () as * const c_uchar ; * outlen = proto . len () as c_uchar ; ffi :: SSL_TLSEXT_ERR_OK } Err (e) => e . 0 , } } }
};
}
