// Generated macro for client_crypto_inner (function)
macro_rules! Depcrate_tests_utilclient_crypto_inner {
() => {
// Module: crate::tests::util
// Provides: {"client_crypto_inner"}
// Dependencies: {}
fn client_crypto_inner (certs : Option < Vec < CertificateDer < 'static > > > , alpn : Option < Vec < Vec < u8 > > > ,) -> QuicClientConfig { let mut roots = rustls :: RootCertStore :: empty () ; for cert in certs . unwrap_or_else (| | vec ! [CERTIFIED_KEY . cert . der () . clone ()]) { roots . add (cert) . unwrap () ; } let mut inner = QuicClientConfig :: inner (WebPkiServerVerifier :: builder_with_provider (Arc :: new (roots) , configured_provider ()) . build () . unwrap () ,) ; inner . key_log = Arc :: new (KeyLogFile :: new ()) ; if let Some (alpn) = alpn { inner . alpn_protocols = alpn ; } inner . try_into () . unwrap () }
};
}
