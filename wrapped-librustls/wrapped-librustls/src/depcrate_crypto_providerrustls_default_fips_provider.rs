// Generated macro for rustls_default_fips_provider (function)
macro_rules! Depcrate_crypto_providerrustls_default_fips_provider {
() => {
// Module: crate::crypto_provider
// Provides: {"rustls_default_fips_provider"}
// Dependencies: {}
# [doc = " Return a `rustls_crypto_provider` that uses FIPS140-3 approved cryptography."] # [doc = ""] # [doc = " Using this function expresses in your code that you require FIPS-approved cryptography,"] # [doc = " and will not compile if you make a mistake with cargo features."] # [doc = ""] # [doc = " See the upstream [rustls FIPS documentation][FIPS] for more information."] # [doc = ""] # [doc = " The caller owns the returned `rustls_crypto_provider` and must free it using"] # [doc = " `rustls_crypto_provider_free`."] # [doc = ""] # [doc = " [FIPS]: https://docs.rs/rustls/latest/rustls/manual/_06_fips/index.html"] # [no_mangle] # [cfg (feature = "fips")] pub extern "C" fn rustls_default_fips_provider () -> * const rustls_crypto_provider { ffi_panic_boundary ! { Arc :: into_raw (Arc :: new (rustls :: crypto :: default_fips_provider ())) as * const rustls_crypto_provider } }
};
}
