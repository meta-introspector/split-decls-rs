// Generated macro for TLS13_CHACHA20_POLY1305_SHA256 (static)
macro_rules! DepcrateTLS13_CHACHA20_POLY1305_SHA256 {
() => {
// Module: crate
// Provides: {"TLS13_CHACHA20_POLY1305_SHA256"}
// Dependencies: {}
pub static TLS13_CHACHA20_POLY1305_SHA256 : & Tls13CipherSuite = & Tls13CipherSuite { common : CipherSuiteCommon { suite : CipherSuite :: TLS13_CHACHA20_POLY1305_SHA256 , hash_provider : & hash :: Sha256 , confidentiality_limit : u64 :: MAX , } , protocol_version : TLS13_VERSION , hkdf_provider : & HkdfUsingHmac (& hmac :: Sha256Hmac) , aead_alg : & aead :: Chacha20Poly1305 , quic : None , } ;
};
}
