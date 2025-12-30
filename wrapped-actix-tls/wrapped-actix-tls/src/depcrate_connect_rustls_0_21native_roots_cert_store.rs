// Generated macro for native_roots_cert_store (function)
macro_rules! Depcrate_connect_rustls_0_21native_roots_cert_store {
() => {
// Module: crate::connect::rustls_0_21
// Provides: {"native_roots_cert_store"}
// Dependencies: {}
# [doc = " Returns root certificates via `rustls-native-certs` crate as a rustls certificate store."] # [doc = ""] # [doc = " See [`rustls_native_certs::load_native_certs()`] for more info on behavior and errors."] # [doc = ""] # [doc = " [`rustls_native_certs::load_native_certs()`]: rustls_native_certs_06::load_native_certs()"] # [cfg (feature = "rustls-0_21-native-roots")] pub fn native_roots_cert_store () -> io :: Result < RootCertStore > { let mut root_certs = RootCertStore :: empty () ; for cert in rustls_native_certs_06 :: load_native_certs () ? { root_certs . add (& tokio_rustls_024 :: rustls :: Certificate (cert . 0)) . unwrap () ; } Ok (root_certs) }
};
}
