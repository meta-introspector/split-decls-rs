// Generated macro for raw_tmp_dh_ssl (function)
macro_rules! Depcrate_ssl_callbacksraw_tmp_dh_ssl {
() => {
// Module: crate::ssl::callbacks
// Provides: {"raw_tmp_dh_ssl"}
// Dependencies: {}
pub unsafe extern "C" fn raw_tmp_dh_ssl < F > (ssl : * mut ffi :: SSL , is_export : c_int , keylength : c_int ,) -> * mut ffi :: DH where F : Fn (& mut SslRef , bool , u32) -> Result < Dh < Params > , ErrorStack > + 'static + Sync + Send , { let ssl = SslRef :: from_ptr_mut (ssl) ; let callback = ssl . ex_data (Ssl :: cached_ex_index :: < Arc < F > > ()) . expect ("BUG: ssl tmp dh callback missing") . clone () ; match callback (ssl , is_export != 0 , keylength as u32) { Ok (dh) => { let ptr = dh . as_ptr () ; mem :: forget (dh) ; ptr } Err (e) => { e . put () ; ptr :: null_mut () } } }
};
}
