// Generated macro for rustls_crypto_provider_load_key (function)
macro_rules! Depcrate_crypto_providerrustls_crypto_provider_load_key {
() => {
// Module: crate::crypto_provider
// Provides: {"rustls_crypto_provider_load_key"}
// Dependencies: {}
# [doc = " Load a private key from the provided PEM content using the crypto provider."] # [doc = ""] # [doc = " `private_key` must point to a buffer of `private_key_len` bytes, containing"] # [doc = " a PEM-encoded private key. The exact formats supported will differ based on"] # [doc = " the crypto provider in use. The default providers support PKCS#1, PKCS#8 or"] # [doc = " SEC1 formats."] # [doc = ""] # [doc = " When this function returns `rustls_result::Ok` a pointer to a `rustls_signing_key`"] # [doc = " is written to `signing_key_out`. The caller owns the returned `rustls_signing_key`"] # [doc = " and must free it with `rustls_signing_key_free`."] # [no_mangle] pub extern "C" fn rustls_crypto_provider_load_key (provider : * const rustls_crypto_provider , private_key : * const u8 , private_key_len : size_t , signing_key_out : * mut * mut rustls_signing_key ,) -> rustls_result { ffi_panic_boundary ! { let provider = try_clone_arc ! (provider) ; let private_key_pem = try_slice ! (private_key , private_key_len) ; let signing_key_out = try_mut_from_ptr_ptr ! (signing_key_out) ; let private_key_der = match PrivateKeyDer :: from_pem_slice (private_key_pem) { Ok (der) => der , Err (_) => return rustls_result :: PrivateKeyParseError , } ; let private_key = match provider . key_provider . load_private_key (private_key_der) { Ok (key) => key , Err (e) => return map_error (e) , } ; set_boxed_mut_ptr (signing_key_out , private_key) ; rustls_result :: Ok } }
};
}
