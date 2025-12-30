// Generated macro for private_key_from_pkey (function)
macro_rules! Depcrate_backend_dhprivate_key_from_pkey {
() => {
// Module: crate::backend::dh
// Provides: {"private_key_from_pkey"}
// Dependencies: {}
pub (crate) fn private_key_from_pkey (pkey : & openssl :: pkey :: PKeyRef < openssl :: pkey :: Private > ,) -> DHPrivateKey { DHPrivateKey { pkey : pkey . to_owned () , } }
};
}
