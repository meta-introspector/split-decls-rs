// Generated macro for macro_717 (macro)
macro_rules! Depcrate_pkeymacro_717 {
() => {
// Module: crate::pkey
// Provides: {"macro_717"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (boringssl , ossl110 , libressl , awslc))] { use ffi :: EVP_PKEY_up_ref ; } else { # [allow (bad_style)] unsafe extern "C" fn EVP_PKEY_up_ref (pkey : * mut ffi :: EVP_PKEY) { ffi :: CRYPTO_add_lock (& mut (* pkey) . references , 1 , ffi :: CRYPTO_LOCK_EVP_PKEY , "pkey.rs\0" . as_ptr () as * const _ , line ! () as c_int ,) ; } } }
};
}
