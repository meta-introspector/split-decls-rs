// Generated macro for generate_key (function)
macro_rules! Depcrate_ed25519generate_key {
() => {
// Module: crate::ed25519
// Provides: {"generate_key"}
// Dependencies: {}
pub (crate) fn generate_key () -> Result < LcPtr < EVP_PKEY > , Unspecified > { LcPtr :: < EVP_PKEY > :: generate (EVP_PKEY_ED25519 , No_EVP_PKEY_CTX_consumer) }
};
}
