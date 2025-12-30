// Generated macro for OCSPResponse (struct)
macro_rules! Depcrate_x509_ocsp_respOCSPResponse {
() => {
// Module: crate::x509::ocsp_resp
// Provides: {"OCSPResponse"}
// Dependencies: {}
# [pyo3 :: pyclass (frozen , module = "cryptography.hazmat.bindings._rust.ocsp")] pub (crate) struct OCSPResponse { raw : Arc < OwnedOCSPResponse > , cached_extensions : pyo3 :: sync :: PyOnceLock < pyo3 :: Py < pyo3 :: PyAny > > , cached_single_extensions : pyo3 :: sync :: PyOnceLock < pyo3 :: Py < pyo3 :: PyAny > > , }
};
}
