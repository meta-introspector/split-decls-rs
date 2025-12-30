// Generated macro for Certificate (struct)
macro_rules! Depcrate_x509_certificateCertificate {
() => {
// Module: crate::x509::certificate
// Provides: {"Certificate"}
// Dependencies: {}
# [pyo3 :: pyclass (frozen , module = "cryptography.hazmat.bindings._rust.x509")] pub (crate) struct Certificate { pub (crate) raw : OwnedCertificate , pub (crate) cached_extensions : pyo3 :: sync :: PyOnceLock < pyo3 :: Py < pyo3 :: PyAny > > , }
};
}
