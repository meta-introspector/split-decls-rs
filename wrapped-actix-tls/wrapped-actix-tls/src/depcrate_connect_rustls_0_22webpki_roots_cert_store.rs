// Generated macro for webpki_roots_cert_store (function)
macro_rules! Depcrate_connect_rustls_0_22webpki_roots_cert_store {
() => {
// Module: crate::connect::rustls_0_22
// Provides: {"webpki_roots_cert_store"}
// Dependencies: {}
# [doc = " Returns standard root certificates from `webpki-roots` crate as a rustls certificate store."] # [cfg (feature = "rustls-0_22-webpki-roots")] pub fn webpki_roots_cert_store () -> tokio_rustls :: rustls :: RootCertStore { let mut root_certs = tokio_rustls :: rustls :: RootCertStore :: empty () ; root_certs . extend (webpki_roots_026 :: TLS_SERVER_ROOTS . to_owned ()) ; root_certs }
};
}
