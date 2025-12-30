// Generated macro for rustls_default_crypto_provider_random (function)
macro_rules! Depcrate_crypto_providerrustls_default_crypto_provider_random {
() => {
// Module: crate::crypto_provider
// Provides: {"rustls_default_crypto_provider_random"}
// Dependencies: {}
# [doc = " Write `len` bytes of cryptographically secure random data to `buff` using the process-wide"] # [doc = " default crypto provider."] # [doc = ""] # [doc = " `buff` must point to a buffer of at least `len` bytes. The caller maintains ownership"] # [doc = " of the buffer."] # [doc = ""] # [doc = " Returns `RUSTLS_RESULT_OK` on success, and one of `RUSTLS_RESULT_NO_DEFAULT_CRYPTO_PROVIDER`"] # [doc = " or `RUSTLS_RESULT_GET_RANDOM_FAILED` on failure."] # [no_mangle] pub extern "C" fn rustls_default_crypto_provider_random (buff : * mut u8 , len : size_t ,) -> rustls_result { ffi_panic_boundary ! { match get_default_or_install_from_crate_features () { Some (provider) => match provider . secure_random . fill (try_slice_mut ! (buff , len)) { Ok (_) => rustls_result :: Ok , Err (_) => rustls_result :: GetRandomFailed , } , None => rustls_result :: NoDefaultCryptoProvider , } } }
};
}
