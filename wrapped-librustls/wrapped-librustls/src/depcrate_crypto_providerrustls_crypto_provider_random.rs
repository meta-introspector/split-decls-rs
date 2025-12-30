// Generated macro for rustls_crypto_provider_random (function)
macro_rules! Depcrate_crypto_providerrustls_crypto_provider_random {
() => {
// Module: crate::crypto_provider
// Provides: {"rustls_crypto_provider_random"}
// Dependencies: {}
# [doc = " Write `len` bytes of cryptographically secure random data to `buff` using the crypto provider."] # [doc = ""] # [doc = " `buff` must point to a buffer of at least `len` bytes. The caller maintains ownership"] # [doc = " of the buffer."] # [doc = ""] # [doc = " Returns `RUSTLS_RESULT_OK` on success, or `RUSTLS_RESULT_GET_RANDOM_FAILED` on failure."] # [no_mangle] pub extern "C" fn rustls_crypto_provider_random (provider : * const rustls_crypto_provider , buff : * mut u8 , len : size_t ,) -> rustls_result { ffi_panic_boundary ! { match try_clone_arc ! (provider) . secure_random . fill (try_slice_mut ! (buff , len)) { Ok (_) => rustls_result :: Ok , Err (_) => rustls_result :: GetRandomFailed , } } }
};
}
