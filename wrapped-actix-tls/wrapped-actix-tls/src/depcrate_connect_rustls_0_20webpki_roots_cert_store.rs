// Generated macro for webpki_roots_cert_store (function)
macro_rules! Depcrate_connect_rustls_0_20webpki_roots_cert_store {
() => {
// Module: crate::connect::rustls_0_20
// Provides: {"webpki_roots_cert_store"}
// Dependencies: {}
# [doc = " Returns standard root certificates from `webpki-roots` crate as a rustls certificate store."] # [cfg (feature = "rustls-0_20-webpki-roots")] pub fn webpki_roots_cert_store () -> RootCertStore { use tokio_rustls_023 :: rustls ; let mut root_certs = RootCertStore :: empty () ; for cert in webpki_roots_022 :: TLS_SERVER_ROOTS . 0 { let cert = rustls :: OwnedTrustAnchor :: from_subject_spki_name_constraints (cert . subject , cert . spki , cert . name_constraints ,) ; let certs = vec ! [cert] . into_iter () ; root_certs . add_server_trust_anchors (certs) ; } root_certs }
};
}
