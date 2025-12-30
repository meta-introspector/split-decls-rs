// Generated macro for rustls_aws_lc_rs_crypto_provider (function)
macro_rules! Depcrate_crypto_providerrustls_aws_lc_rs_crypto_provider {
() => {
// Module: crate::crypto_provider
// Provides: {"rustls_aws_lc_rs_crypto_provider"}
// Dependencies: {}
# [doc = " Return the `rustls_crypto_provider` backed by the `aws-lc-rs` cryptography library."] # [doc = ""] # [doc = " The caller owns the returned `rustls_crypto_provider` and must free it using"] # [doc = " `rustls_crypto_provider_free`."] # [no_mangle] # [cfg (feature = "aws-lc-rs")] pub extern "C" fn rustls_aws_lc_rs_crypto_provider () -> * const rustls_crypto_provider { ffi_panic_boundary ! { Arc :: into_raw (Arc :: new (aws_lc_rs :: default_provider ())) as * const rustls_crypto_provider } }
};
}
