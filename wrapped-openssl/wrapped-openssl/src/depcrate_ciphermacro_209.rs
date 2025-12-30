// Generated macro for macro_209 (macro)
macro_rules! Depcrate_ciphermacro_209 {
() => {
// Module: crate::cipher
// Provides: {"macro_209"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (boringssl , ossl110 , libressl , awslc))] { use ffi :: { EVP_CIPHER_block_size , EVP_CIPHER_iv_length , EVP_CIPHER_key_length } ; } else { use libc :: c_int ; # [allow (bad_style)] pub unsafe fn EVP_CIPHER_iv_length (ptr : * const ffi :: EVP_CIPHER) -> c_int { (* ptr) . iv_len } # [allow (bad_style)] pub unsafe fn EVP_CIPHER_block_size (ptr : * const ffi :: EVP_CIPHER) -> c_int { (* ptr) . block_size } # [allow (bad_style)] pub unsafe fn EVP_CIPHER_key_length (ptr : * const ffi :: EVP_CIPHER) -> c_int { (* ptr) . key_len } } }
};
}
