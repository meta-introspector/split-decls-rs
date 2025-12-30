// Generated macro for private_key_from_pkey (function)
macro_rules! Depcrate_backend_dsaprivate_key_from_pkey {
() => {
// Module: crate::backend::dsa
// Provides: {"private_key_from_pkey"}
// Dependencies: {}
pub (crate) fn private_key_from_pkey (pkey : & openssl :: pkey :: PKeyRef < openssl :: pkey :: Private > ,) -> DsaPrivateKey { DsaPrivateKey { pkey : pkey . to_owned () , } }
};
}
