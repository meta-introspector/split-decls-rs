// Generated macro for make_evp_cipher_ctx_basic (function)
macro_rules! Depcrate_crypto_openssl_quictlsmake_evp_cipher_ctx_basic {
() => {
// Module: crate::crypto::openssl_quictls
// Provides: {"make_evp_cipher_ctx_basic"}
// Dependencies: {}
fn make_evp_cipher_ctx_basic (alg : Algorithm , aead : bool , enc : u32 ,) -> Result < * mut EVP_CIPHER_CTX > { let ctx : * mut EVP_CIPHER_CTX = unsafe { let cipher : * const EVP_AEAD = if aead { alg . get_evp_aead () } else { alg . get_evp () } ; let ctx = EVP_CIPHER_CTX_new () ; if ctx . is_null () { return Err (Error :: CryptoFail) ; } let rc = EVP_CipherInit_ex2 (ctx , cipher , std :: ptr :: null_mut () , std :: ptr :: null_mut () , enc as c_int , std :: ptr :: null () ,) ; if rc != 1 { return Err (Error :: CryptoFail) ; } ctx } ; Ok (ctx) }
};
}
