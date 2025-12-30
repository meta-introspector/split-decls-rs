// Generated macro for TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256 (static)
macro_rules! DepcrateTLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256 {
() => {
// Module: crate
// Provides: {"TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256"}
// Dependencies: {}
pub static TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256 : & Tls12CipherSuite = & Tls12CipherSuite { common : CipherSuiteCommon { suite : CipherSuite :: TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256 , hash_provider : & hash :: Sha256 , confidentiality_limit : u64 :: MAX , } , protocol_version : TLS12_VERSION , kx : KeyExchangeAlgorithm :: ECDHE , sign : & [SignatureScheme :: RSA_PSS_SHA256 , SignatureScheme :: RSA_PKCS1_SHA256 ,] , prf_provider : & PrfUsingHmac (& hmac :: Sha256Hmac) , aead_alg : & aead :: Chacha20Poly1305 , } ;
};
}
