// Generated macro for public_key_from_pkey (function)
macro_rules! Depcrate_backend_dsapublic_key_from_pkey {
() => {
// Module: crate::backend::dsa
// Provides: {"public_key_from_pkey"}
// Dependencies: {}
pub (crate) fn public_key_from_pkey (pkey : & openssl :: pkey :: PKeyRef < openssl :: pkey :: Public > ,) -> DsaPublicKey { DsaPublicKey { pkey : pkey . to_owned () , } }
};
}
