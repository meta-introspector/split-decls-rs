// Generated macro for public_key_from_pkey (function)
macro_rules! Depcrate_backend_ed25519public_key_from_pkey {
() => {
// Module: crate::backend::ed25519
// Provides: {"public_key_from_pkey"}
// Dependencies: {}
pub (crate) fn public_key_from_pkey (pkey : & openssl :: pkey :: PKeyRef < openssl :: pkey :: Public > ,) -> Ed25519PublicKey { Ed25519PublicKey { pkey : pkey . to_owned () , } }
};
}
