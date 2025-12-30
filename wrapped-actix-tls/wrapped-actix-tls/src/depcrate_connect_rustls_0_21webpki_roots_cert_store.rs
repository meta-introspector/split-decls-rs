// Generated macro for webpki_roots_cert_store (function)
macro_rules! Depcrate_connect_rustls_0_21webpki_roots_cert_store {
() => {
// Module: crate::connect::rustls_0_21
// Provides: {"webpki_roots_cert_store"}
// Dependencies: {}
# [doc = " Returns standard root certificates from `webpki-roots` crate as a rustls certificate store."] # [cfg (feature = "rustls-0_21-webpki-roots")] pub fn webpki_roots_cert_store () -> RootCertStore { use tokio_rustls_024 :: rustls ; let mut root_certs = RootCertStore :: empty () ; for cert in webpki_roots_025 :: TLS_SERVER_ROOTS { let cert = rustls :: OwnedTrustAnchor :: from_subject_spki_name_constraints (cert . subject , cert . spki , cert . name_constraints ,) ; let certs = vec ! [cert] . into_iter () ; root_certs . add_trust_anchors (certs) ; } root_certs }
};
}
