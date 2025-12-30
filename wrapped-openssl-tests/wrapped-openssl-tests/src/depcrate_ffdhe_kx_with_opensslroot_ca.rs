// Generated macro for root_ca (function)
macro_rules! Depcrate_ffdhe_kx_with_opensslroot_ca {
() => {
// Module: crate::ffdhe_kx_with_openssl
// Provides: {"root_ca"}
// Dependencies: {}
fn root_ca () -> RootCertStore { let mut res = RootCertStore :: empty () ; res . add_parsable_certificates ([CertificateDer :: from (fs :: read (CA_FILE) . unwrap ())]) ; res }
};
}
