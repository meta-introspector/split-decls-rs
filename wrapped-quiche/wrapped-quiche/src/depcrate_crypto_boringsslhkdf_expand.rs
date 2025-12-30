// Generated macro for hkdf_expand (function)
macro_rules! Depcrate_crypto_boringsslhkdf_expand {
() => {
// Module: crate::crypto::boringssl
// Provides: {"hkdf_expand"}
// Dependencies: {}
pub (crate) fn hkdf_expand (alg : Algorithm , out : & mut [u8] , secret : & [u8] , info : & [u8] ,) -> Result < () > { let rc = unsafe { HKDF_expand (out . as_mut_ptr () , out . len () , alg . get_evp_digest () , secret . as_ptr () , secret . len () , info . as_ptr () , info . len () ,) } ; if rc != 1 { return Err (Error :: CryptoFail) ; } Ok (()) }
};
}
