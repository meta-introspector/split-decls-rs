// Generated macro for public_key_from_pkey (function)
macro_rules! Depcrate_backend_x25519public_key_from_pkey {
() => {
// Module: crate::backend::x25519
// Provides: {"public_key_from_pkey"}
// Dependencies: {}
pub (crate) fn public_key_from_pkey (pkey : & openssl :: pkey :: PKeyRef < openssl :: pkey :: Public > ,) -> X25519PublicKey { X25519PublicKey { pkey : pkey . to_owned () , } }
};
}
