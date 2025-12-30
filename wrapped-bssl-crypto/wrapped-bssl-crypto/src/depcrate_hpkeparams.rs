// Generated macro for Params (struct)
macro_rules! Depcrate_hpkeParams {
() => {
// Module: crate::hpke
// Provides: {"Params"}
// Dependencies: {}
# [doc = " HPKE parameters, including KEM, KDF, and AEAD."] pub struct Params { kem : * const bssl_sys :: EVP_HPKE_KEM , kdf : * const bssl_sys :: EVP_HPKE_KDF , aead : * const bssl_sys :: EVP_HPKE_AEAD , }
};
}
