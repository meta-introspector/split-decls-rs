// Generated macro for hkdf_extract (function)
macro_rules! Depcrate_crypto_openssl_quictlshkdf_extract {
() => {
// Module: crate::crypto::openssl_quictls
// Provides: {"hkdf_extract"}
// Dependencies: {}
pub (crate) fn hkdf_extract (alg : Algorithm , out : & mut [u8] , secret : & [u8] , salt : & [u8] ,) -> Result < () > { let mut out_len = out . len () ; unsafe { let prf = alg . get_evp_digest () ; let ctx = EVP_PKEY_CTX_new_id (1036 , std :: ptr :: null_mut () ,) ; if EVP_PKEY_derive_init (ctx) != 1 || EVP_PKEY_CTX_set_hkdf_mode (ctx , 1 ,) != 1 || EVP_PKEY_CTX_set_hkdf_md (ctx , prf) != 1 || EVP_PKEY_CTX_set1_hkdf_salt (ctx , salt . as_ptr () , salt . len ()) != 1 || EVP_PKEY_CTX_set1_hkdf_key (ctx , secret . as_ptr () , secret . len ()) != 1 || EVP_PKEY_derive (ctx , out . as_mut_ptr () , & mut out_len) != 1 { EVP_PKEY_CTX_free (ctx) ; return Err (Error :: CryptoFail) ; } EVP_PKEY_CTX_free (ctx) ; } Ok (()) }
};
}
