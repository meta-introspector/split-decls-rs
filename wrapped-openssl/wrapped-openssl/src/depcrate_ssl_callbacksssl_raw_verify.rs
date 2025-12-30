// Generated macro for ssl_raw_verify (function)
macro_rules! Depcrate_ssl_callbacksssl_raw_verify {
() => {
// Module: crate::ssl::callbacks
// Provides: {"ssl_raw_verify"}
// Dependencies: {}
pub extern "C" fn ssl_raw_verify < F > (preverify_ok : c_int , x509_ctx : * mut ffi :: X509_STORE_CTX ,) -> c_int where F : Fn (bool , & mut X509StoreContextRef) -> bool + 'static + Sync + Send , { unsafe { let ctx = X509StoreContextRef :: from_ptr_mut (x509_ctx) ; let ssl_idx = X509StoreContext :: ssl_idx () . expect ("BUG: store context ssl index missing") ; let callback_idx = Ssl :: cached_ex_index :: < Arc < F > > () ; let callback = ctx . ex_data (ssl_idx) . expect ("BUG: store context missing ssl") . ex_data (callback_idx) . expect ("BUG: ssl verify callback missing") . clone () ; callback (preverify_ok != 0 , ctx) as c_int } }
};
}
