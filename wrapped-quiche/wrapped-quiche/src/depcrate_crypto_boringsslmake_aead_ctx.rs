// Generated macro for make_aead_ctx (function)
macro_rules! Depcrate_crypto_boringsslmake_aead_ctx {
() => {
// Module: crate::crypto::boringssl
// Provides: {"make_aead_ctx"}
// Dependencies: {}
fn make_aead_ctx (alg : Algorithm , key : & [u8]) -> Result < EVP_AEAD_CTX > { let mut ctx = MaybeUninit :: uninit () ; let ctx = unsafe { let aead = alg . get_evp_aead () ; let rc = EVP_AEAD_CTX_init (ctx . as_mut_ptr () , aead , key . as_ptr () , alg . key_len () , alg . tag_len () , std :: ptr :: null_mut () ,) ; if rc != 1 { return Err (Error :: CryptoFail) ; } ctx . assume_init () } ; Ok (ctx) }
};
}
