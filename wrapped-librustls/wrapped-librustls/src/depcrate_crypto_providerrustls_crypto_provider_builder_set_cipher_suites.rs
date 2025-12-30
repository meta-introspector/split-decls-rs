// Generated macro for rustls_crypto_provider_builder_set_cipher_suites (function)
macro_rules! Depcrate_crypto_providerrustls_crypto_provider_builder_set_cipher_suites {
() => {
// Module: crate::crypto_provider
// Provides: {"rustls_crypto_provider_builder_set_cipher_suites"}
// Dependencies: {}
# [doc = " Customize the supported ciphersuites of the `rustls_crypto_provider_builder`."] # [doc = ""] # [doc = " Returns an error if the builder has already been built. Overwrites any previously"] # [doc = " set ciphersuites."] # [no_mangle] pub extern "C" fn rustls_crypto_provider_builder_set_cipher_suites (builder : * mut rustls_crypto_provider_builder , cipher_suites : * const * const rustls_supported_ciphersuite , cipher_suites_len : size_t ,) -> rustls_result { ffi_panic_boundary ! { let builder = try_mut_from_ptr ! (builder) ; let builder = match builder { Some (builder) => builder , None => return rustls_result :: AlreadyUsed , } ; let cipher_suites = try_slice ! (cipher_suites , cipher_suites_len) ; let mut supported_cipher_suites = Vec :: new () ; for cs in cipher_suites { let cs = * cs ; let cs = try_ref_from_ptr ! (cs) ; supported_cipher_suites . push (* cs) ; } builder . cipher_suites = supported_cipher_suites ; rustls_result :: Ok } }
};
}
