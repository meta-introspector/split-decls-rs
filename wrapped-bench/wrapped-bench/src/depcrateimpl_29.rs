// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
impl CipherSuite { pub fn as_rustls (self) -> rustls :: SupportedCipherSuite { use rustls :: crypto :: ring :: cipher_suite ; match self { CipherSuite :: Aes128 => cipher_suite :: TLS13_AES_128_GCM_SHA256 , CipherSuite :: Aes256 => cipher_suite :: TLS13_AES_256_GCM_SHA384 , CipherSuite :: Chacha20 => cipher_suite :: TLS13_CHACHA20_POLY1305_SHA256 , } } }
};
}
