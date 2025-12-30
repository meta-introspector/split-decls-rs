// Generated macro for raw_tmp_dh (function)
macro_rules! Depcrate_ssl_callbacksraw_tmp_dh {
() => {
// Module: crate::ssl::callbacks
// Provides: {"raw_tmp_dh"}
// Dependencies: {}
pub unsafe extern "C" fn raw_tmp_dh < F > (ssl : * mut ffi :: SSL , is_export : c_int , keylength : c_int ,) -> * mut ffi :: DH where F : Fn (& mut SslRef , bool , u32) -> Result < Dh < Params > , ErrorStack > + 'static + Sync + Send , { let ssl = SslRef :: from_ptr_mut (ssl) ; let callback = ssl . ssl_context () . ex_data (SslContext :: cached_ex_index :: < F > ()) . expect ("BUG: tmp dh callback missing") as * const F ; match (* callback) (ssl , is_export != 0 , keylength as u32) { Ok (dh) => { let ptr = dh . as_ptr () ; mem :: forget (dh) ; ptr } Err (e) => { e . put () ; ptr :: null_mut () } } }
};
}
