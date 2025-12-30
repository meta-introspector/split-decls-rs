// Generated macro for raw_verify (function)
macro_rules! Depcrate_ssl_callbacksraw_verify {
() => {
// Module: crate::ssl::callbacks
// Provides: {"raw_verify"}
// Dependencies: {}
pub extern "C" fn raw_verify < F > (preverify_ok : c_int , x509_ctx : * mut ffi :: X509_STORE_CTX) -> c_int where F : Fn (bool , & mut X509StoreContextRef) -> bool + 'static + Sync + Send , { unsafe { let ctx = X509StoreContextRef :: from_ptr_mut (x509_ctx) ; let ssl_idx = X509StoreContext :: ssl_idx () . expect ("BUG: store context ssl index missing") ; let verify_idx = SslContext :: cached_ex_index :: < F > () ; let verify = ctx . ex_data (ssl_idx) . expect ("BUG: store context missing ssl") . ssl_context () . ex_data (verify_idx) . expect ("BUG: verify callback missing") as * const F ; (* verify) (preverify_ok != 0 , ctx) as c_int } }
};
}
