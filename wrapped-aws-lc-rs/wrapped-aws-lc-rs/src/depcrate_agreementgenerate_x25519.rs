// Generated macro for generate_x25519 (function)
macro_rules! Depcrate_agreementgenerate_x25519 {
() => {
// Module: crate::agreement
// Provides: {"generate_x25519"}
// Dependencies: {}
pub (crate) fn generate_x25519 () -> Result < LcPtr < EVP_PKEY > , Unspecified > { LcPtr :: < EVP_PKEY > :: generate (EVP_PKEY_X25519 , No_EVP_PKEY_CTX_consumer) }
};
}
