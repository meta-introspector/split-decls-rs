// Generated macro for macro_1336 (macro)
macro_rules! Depcrate_symmmacro_1336 {
() => {
// Module: crate::symm
// Provides: {"macro_1336"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (boringssl , ossl110 , libressl , awslc))] { use ffi :: { EVP_CIPHER_block_size , EVP_CIPHER_iv_length , EVP_CIPHER_key_length } ; } else { use crate :: LenType ; # [allow (bad_style)] pub unsafe fn EVP_CIPHER_iv_length (ptr : * const ffi :: EVP_CIPHER) -> LenType { (* ptr) . iv_len } # [allow (bad_style)] pub unsafe fn EVP_CIPHER_block_size (ptr : * const ffi :: EVP_CIPHER) -> LenType { (* ptr) . block_size } # [allow (bad_style)] pub unsafe fn EVP_CIPHER_key_length (ptr : * const ffi :: EVP_CIPHER) -> LenType { (* ptr) . key_len } } }
};
}
