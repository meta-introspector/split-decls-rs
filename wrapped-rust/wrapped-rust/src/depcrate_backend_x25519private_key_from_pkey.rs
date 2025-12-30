// Generated macro for private_key_from_pkey (function)
macro_rules! Depcrate_backend_x25519private_key_from_pkey {
() => {
// Module: crate::backend::x25519
// Provides: {"private_key_from_pkey"}
// Dependencies: {}
pub (crate) fn private_key_from_pkey (pkey : & openssl :: pkey :: PKeyRef < openssl :: pkey :: Private > ,) -> X25519PrivateKey { X25519PrivateKey { pkey : pkey . to_owned () , } }
};
}
