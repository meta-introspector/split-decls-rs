// Generated macro for ALL_SUPPORTED_SUITES (static)
macro_rules! Depcrate_hpkeALL_SUPPORTED_SUITES {
() => {
// Module: crate::hpke
// Provides: {"ALL_SUPPORTED_SUITES"}
// Dependencies: {}
# [doc = " All supported HPKE suites."] # [doc = ""] # [doc = " Note: hpke-rs w/ rust-crypto does not support P-384 and P-521 DH KEMs."] pub static ALL_SUPPORTED_SUITES : & [& dyn Hpke] = & [DHKEM_P256_HKDF_SHA256_AES_128 , DHKEM_P256_HKDF_SHA256_AES_256 , DHKEM_P256_HKDF_SHA256_CHACHA20_POLY1305 , DHKEM_X25519_HKDF_SHA256_AES_128 , DHKEM_X25519_HKDF_SHA256_AES_256 , DHKEM_X25519_HKDF_SHA256_CHACHA20_POLY1305 ,] ;
};
}
