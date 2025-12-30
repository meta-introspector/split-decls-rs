// Generated macro for native_roots_cert_store (function)
macro_rules! Depcrate_connect_rustls_0_23native_roots_cert_store {
() => {
// Module: crate::connect::rustls_0_23
// Provides: {"native_roots_cert_store"}
// Dependencies: {}
# [doc = " Returns root certificates via `rustls-native-certs` crate as a rustls certificate store."] # [doc = ""] # [doc = " See [`rustls_native_certs::load_native_certs()`] for more info on behavior and errors."] # [doc = ""] # [doc = " [`rustls_native_certs::load_native_certs()`]: rustls_native_certs_08::load_native_certs()"] # [cfg (feature = "rustls-0_23-native-roots")] pub fn native_roots_cert_store () -> io :: Result < tokio_rustls :: rustls :: RootCertStore > { let mut root_certs = tokio_rustls :: rustls :: RootCertStore :: empty () ; let certs = rustls_native_certs_08 :: load_native_certs () ; if let Some (err) = certs . errors . into_iter () . next () { return Err (io :: Error :: other (err)) ; } for cert in certs . certs { root_certs . add (cert) . unwrap () ; } Ok (root_certs) }
};
}
