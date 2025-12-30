// Generated macro for impl_911 (impl)
macro_rules! Depcrate_x509_crlimpl_911 {
() => {
// Module: crate::x509::crl
// Provides: {"impl_911"}
// Dependencies: {}
impl OwnedRevokedCertificate { fn clone_with_py (& self , py : pyo3 :: Python < '_ >) -> OwnedRevokedCertificate { map_revoked_cert (self , py , | cert | cert . clone ()) } }
};
}
