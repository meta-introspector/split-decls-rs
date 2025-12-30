// Generated macro for server_crypto_inner (function)
macro_rules! Depcrate_tests_utilserver_crypto_inner {
() => {
// Module: crate::tests::util
// Provides: {"server_crypto_inner"}
// Dependencies: {}
fn server_crypto_inner (identity : Option < (CertificateDer < 'static > , PrivateKeyDer < 'static >) > , alpn : Option < Vec < Vec < u8 > > > ,) -> QuicServerConfig { let (cert , key) = identity . unwrap_or_else (| | { (CERTIFIED_KEY . cert . der () . clone () , PrivateKeyDer :: Pkcs8 (CERTIFIED_KEY . signing_key . serialize_der () . into ()) ,) }) ; let mut config = QuicServerConfig :: inner (vec ! [cert] , key) . unwrap () ; if let Some (alpn) = alpn { config . alpn_protocols = alpn ; } config . try_into () . unwrap () }
};
}
