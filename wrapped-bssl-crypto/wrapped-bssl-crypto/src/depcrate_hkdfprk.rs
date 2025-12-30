// Generated macro for Prk (struct)
macro_rules! Depcrate_hkdfPrk {
() => {
// Module: crate::hkdf
// Provides: {"Prk"}
// Dependencies: {}
# [doc = " A pseudo-random key, an intermediate value in the HKDF computation."] pub struct Prk { prk : [u8 ; bssl_sys :: EVP_MAX_MD_SIZE as usize] , len : usize , evp_md : * const bssl_sys :: EVP_MD , }
};
}
