// Generated macro for rustls_supported_hpke (function)
macro_rules! Depcrate_crypto_providerrustls_supported_hpke {
() => {
// Module: crate::crypto_provider
// Provides: {"rustls_supported_hpke"}
// Dependencies: {}
# [doc = " Returns a pointer to the supported `rustls_hpke` Hybrid Public Key Encryption (HPKE)"] # [doc = " suites, or `NULL` if HPKE is not supported."] # [doc = ""] # [doc = " HPKE is only supported with the `aws-lc-rs` cryptography provider."] # [doc = ""] # [doc = " The returned pointer has a static lifetime equal to that of the program and does not"] # [doc = " need to be freed."] # [no_mangle] pub extern "C" fn rustls_supported_hpke () -> * const rustls_hpke { ffi_panic_boundary ! { # [cfg (feature = "aws-lc-rs")] { AWS_LC_RS_HPKE as * const _ as _ } # [cfg (not (feature = "aws-lc-rs"))] { core :: ptr :: null () } } }
};
}
