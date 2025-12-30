// Generated macro for rustls_crypto_provider_fips (function)
macro_rules! Depcrate_crypto_providerrustls_crypto_provider_fips {
() => {
// Module: crate::crypto_provider
// Provides: {"rustls_crypto_provider_fips"}
// Dependencies: {}
# [doc = " Returns true if the `rustls_crypto_provider` is operating in FIPS mode."] # [doc = ""] # [doc = " This covers only the cryptographic parts of FIPS approval. There are also"] # [doc = " TLS protocol-level recommendations made by NIST. You should prefer to call"] # [doc = " `rustls_client_config_fips` or `rustls_server_config_fips` which take these"] # [doc = " into account."] # [no_mangle] pub extern "C" fn rustls_crypto_provider_fips (provider : * const rustls_crypto_provider) -> bool { ffi_panic_boundary ! { try_ref_from_ptr ! (provider) . fips () } }
};
}
