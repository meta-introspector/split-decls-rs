// Generated macro for TLS_DHE_RSA_WITH_AES_128_GCM_SHA256 (static)
macro_rules! Depcrate_ffdheTLS_DHE_RSA_WITH_AES_128_GCM_SHA256 {
() => {
// Module: crate::ffdhe
// Provides: {"TLS_DHE_RSA_WITH_AES_128_GCM_SHA256"}
// Dependencies: {}
pub (crate) static TLS_DHE_RSA_WITH_AES_128_GCM_SHA256 : Tls12CipherSuite = Tls12CipherSuite { common : CipherSuiteCommon { suite : CipherSuite :: TLS_DHE_RSA_WITH_AES_128_GCM_SHA256 , .. provider :: cipher_suite :: TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256 . common } , kx : KeyExchangeAlgorithm :: DHE , .. * provider :: cipher_suite :: TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256 } ;
};
}
