// Generated macro for macro_1238 (macro)
macro_rules! Depcrate_sslmacro_1238 {
() => {
// Module: crate::ssl
// Provides: {"macro_1238"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (boringssl , ossl110 , libressl , awslc))] { use ffi :: { SSL_CTX_up_ref , SSL_SESSION_get_master_key , SSL_SESSION_up_ref , SSL_is_server } ; } else { # [allow (bad_style)] pub unsafe fn SSL_CTX_up_ref (ssl : * mut ffi :: SSL_CTX) -> c_int { ffi :: CRYPTO_add_lock (& mut (* ssl) . references , 1 , ffi :: CRYPTO_LOCK_SSL_CTX , "mod.rs\0" . as_ptr () as * const _ , line ! () as c_int ,) ; 0 } # [allow (bad_style)] pub unsafe fn SSL_SESSION_get_master_key (session : * const ffi :: SSL_SESSION , out : * mut c_uchar , mut outlen : usize ,) -> usize { if outlen == 0 { return (* session) . master_key_length as usize ; } if outlen > (* session) . master_key_length as usize { outlen = (* session) . master_key_length as usize ; } ptr :: copy_nonoverlapping ((* session) . master_key . as_ptr () , out , outlen) ; outlen } # [allow (bad_style)] pub unsafe fn SSL_is_server (s : * mut ffi :: SSL) -> c_int { (* s) . server } # [allow (bad_style)] pub unsafe fn SSL_SESSION_up_ref (ses : * mut ffi :: SSL_SESSION) -> c_int { ffi :: CRYPTO_add_lock (& mut (* ses) . references , 1 , ffi :: CRYPTO_LOCK_SSL_CTX , "mod.rs\0" . as_ptr () as * const _ , line ! () as c_int ,) ; 0 } } }
};
}
