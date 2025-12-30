// Generated macro for public_key_from_pkey (function)
macro_rules! Depcrate_backend_dhpublic_key_from_pkey {
() => {
// Module: crate::backend::dh
// Provides: {"public_key_from_pkey"}
// Dependencies: {}
pub (crate) fn public_key_from_pkey (pkey : & openssl :: pkey :: PKeyRef < openssl :: pkey :: Public > ,) -> DHPublicKey { DHPublicKey { pkey : pkey . to_owned () , } }
};
}
