// Generated macro for impl_406 (impl)
macro_rules! Depcrate_crypto_openssl_quictlsimpl_406 {
() => {
// Module: crate::crypto::openssl_quictls
// Provides: {"impl_406"}
// Dependencies: {}
impl Clone for HeaderProtectionKey { fn clone (& self) -> Self { let ctx = unsafe { EVP_CIPHER_CTX_dup (self . ctx) } ; Self { ctx , key : self . key . clone () , } } }
};
}
