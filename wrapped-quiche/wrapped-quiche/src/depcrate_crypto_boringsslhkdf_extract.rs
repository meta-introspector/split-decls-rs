// Generated macro for hkdf_extract (function)
macro_rules! Depcrate_crypto_boringsslhkdf_extract {
() => {
// Module: crate::crypto::boringssl
// Provides: {"hkdf_extract"}
// Dependencies: {}
pub (crate) fn hkdf_extract (alg : Algorithm , out : & mut [u8] , secret : & [u8] , salt : & [u8] ,) -> Result < () > { let mut out_len = out . len () ; let rc = unsafe { HKDF_extract (out . as_mut_ptr () , & mut out_len , alg . get_evp_digest () , secret . as_ptr () , secret . len () , salt . as_ptr () , salt . len () ,) } ; if rc != 1 { return Err (Error :: CryptoFail) ; } Ok (()) }
};
}
