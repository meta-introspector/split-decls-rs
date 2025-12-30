// Generated macro for Ed25519KeyPair (struct)
macro_rules! Depcrate_ed25519Ed25519KeyPair {
() => {
// Module: crate::ed25519
// Provides: {"Ed25519KeyPair"}
// Dependencies: {}
# [doc = " An Ed25519 key pair, for signing."] # [allow (clippy :: module_name_repetitions)] pub struct Ed25519KeyPair { evp_pkey : LcPtr < EVP_PKEY > , public_key : PublicKey , }
};
}
