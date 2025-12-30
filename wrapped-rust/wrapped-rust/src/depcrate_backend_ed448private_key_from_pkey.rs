// Generated macro for private_key_from_pkey (function)
macro_rules! Depcrate_backend_ed448private_key_from_pkey {
() => {
// Module: crate::backend::ed448
// Provides: {"private_key_from_pkey"}
// Dependencies: {}
pub (crate) fn private_key_from_pkey (pkey : & openssl :: pkey :: PKeyRef < openssl :: pkey :: Private > ,) -> Ed448PrivateKey { Ed448PrivateKey { pkey : pkey . to_owned () , } }
};
}
