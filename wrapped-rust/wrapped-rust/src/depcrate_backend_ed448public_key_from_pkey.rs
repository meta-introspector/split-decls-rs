// Generated macro for public_key_from_pkey (function)
macro_rules! Depcrate_backend_ed448public_key_from_pkey {
() => {
// Module: crate::backend::ed448
// Provides: {"public_key_from_pkey"}
// Dependencies: {}
pub (crate) fn public_key_from_pkey (pkey : & openssl :: pkey :: PKeyRef < openssl :: pkey :: Public > ,) -> Ed448PublicKey { Ed448PublicKey { pkey : pkey . to_owned () , } }
};
}
