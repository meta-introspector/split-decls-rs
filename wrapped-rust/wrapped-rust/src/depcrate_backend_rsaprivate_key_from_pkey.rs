// Generated macro for private_key_from_pkey (function)
macro_rules! Depcrate_backend_rsaprivate_key_from_pkey {
() => {
// Module: crate::backend::rsa
// Provides: {"private_key_from_pkey"}
// Dependencies: {}
pub (crate) fn private_key_from_pkey (pkey : & openssl :: pkey :: PKeyRef < openssl :: pkey :: Private > , unsafe_skip_rsa_key_validation : bool ,) -> CryptographyResult < RsaPrivateKey > { if ! unsafe_skip_rsa_key_validation { check_rsa_private_key (& pkey . rsa () . unwrap ()) ? ; } Ok (RsaPrivateKey { pkey : pkey . to_owned () , }) }
};
}
