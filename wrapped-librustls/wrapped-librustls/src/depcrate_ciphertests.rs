// Generated macro for tests (module)
macro_rules! Depcrate_ciphertests {
() => {
// Module: crate::cipher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (test , any (feature = "ring" , feature = "aws-lc-rs")))] mod tests { use crate :: crypto_provider :: { rustls_default_crypto_provider_ciphersuites_get , rustls_default_crypto_provider_ciphersuites_len , } ; use super :: * ; # [test] fn default_cipher_suites () { let num_suites = rustls_default_crypto_provider_ciphersuites_len () ; assert ! (num_suites > 2) ; for i in 0 .. num_suites { let suite = rustls_default_crypto_provider_ciphersuites_get (i) ; let name = rustls_supported_ciphersuite_get_name (suite) ; let name = unsafe { name . to_str () } ; let proto = rustls_supported_ciphersuite_protocol_version (suite) ; println ! ("{i}: {name} {proto:?}") ; } } }
};
}
