// Generated macro for CertificateRevocationList (struct)
macro_rules! Depcrate_x509_crlCertificateRevocationList {
() => {
// Module: crate::x509::crl
// Provides: {"CertificateRevocationList"}
// Dependencies: {}
# [pyo3 :: pyclass (frozen , module = "cryptography.hazmat.bindings._rust.x509")] pub (crate) struct CertificateRevocationList { owned : OwnedCertificateRevocationList , revoked_certs : pyo3 :: sync :: PyOnceLock < Vec < OwnedRevokedCertificate > > , cached_extensions : pyo3 :: sync :: PyOnceLock < pyo3 :: Py < pyo3 :: PyAny > > , }
};
}
