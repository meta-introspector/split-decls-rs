// Generated macro for public_key_from_pkey (function)
macro_rules! Depcrate_backend_x448public_key_from_pkey {
() => {
// Module: crate::backend::x448
// Provides: {"public_key_from_pkey"}
// Dependencies: {}
pub (crate) fn public_key_from_pkey (pkey : & openssl :: pkey :: PKeyRef < openssl :: pkey :: Public > ,) -> X448PublicKey { X448PublicKey { pkey : pkey . to_owned () , } }
};
}
