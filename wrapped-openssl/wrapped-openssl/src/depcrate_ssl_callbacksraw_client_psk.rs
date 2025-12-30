// Generated macro for raw_client_psk (function)
macro_rules! Depcrate_ssl_callbacksraw_client_psk {
() => {
// Module: crate::ssl::callbacks
// Provides: {"raw_client_psk"}
// Dependencies: {}
# [cfg (not (osslconf = "OPENSSL_NO_PSK"))] pub extern "C" fn raw_client_psk < F > (ssl : * mut ffi :: SSL , hint : * const c_char , identity : * mut c_char , max_identity_len : c_uint , psk : * mut c_uchar , max_psk_len : c_uint ,) -> c_uint where F : Fn (& mut SslRef , Option < & [u8] > , & mut [u8] , & mut [u8]) -> Result < usize , ErrorStack > + 'static + Sync + Send , { unsafe { let ssl = SslRef :: from_ptr_mut (ssl) ; let callback_idx = SslContext :: cached_ex_index :: < F > () ; let callback = ssl . ssl_context () . ex_data (callback_idx) . expect ("BUG: psk callback missing") as * const F ; let hint = if ! hint . is_null () { Some (CStr :: from_ptr (hint) . to_bytes ()) } else { None } ; let identity_sl = util :: from_raw_parts_mut (identity as * mut u8 , max_identity_len as usize) ; # [allow (clippy :: unnecessary_cast)] let psk_sl = util :: from_raw_parts_mut (psk as * mut u8 , max_psk_len as usize) ; match (* callback) (ssl , hint , identity_sl , psk_sl) { Ok (psk_len) => psk_len as u32 , Err (e) => { e . put () ; 0 } } } }
};
}
