// Generated macro for impl_401 (impl)
macro_rules! Depcrate_crypto_openssl_quictlsimpl_401 {
() => {
// Module: crate::crypto::openssl_quictls
// Provides: {"impl_401"}
// Dependencies: {}
impl Drop for PacketKey { fn drop (& mut self) { unsafe { EVP_CIPHER_CTX_free (self . ctx) } } }
};
}
