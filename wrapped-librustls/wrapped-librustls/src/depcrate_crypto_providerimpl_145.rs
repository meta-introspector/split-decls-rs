// Generated macro for impl_145 (impl)
macro_rules! Depcrate_crypto_providerimpl_145 {
() => {
// Module: crate::crypto_provider
// Provides: {"impl_145"}
// Dependencies: {}
impl CryptoProviderBuilder { fn build_provider (self) -> CryptoProvider { let cipher_suites = match self . cipher_suites . is_empty () { true => self . base . cipher_suites . clone () , false => self . cipher_suites , } ; CryptoProvider { cipher_suites , kx_groups : self . base . kx_groups . clone () , signature_verification_algorithms : self . base . signature_verification_algorithms , secure_random : self . base . secure_random , key_provider : self . base . key_provider , } } }
};
}
