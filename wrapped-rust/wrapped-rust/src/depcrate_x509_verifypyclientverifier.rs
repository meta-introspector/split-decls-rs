// Generated macro for PyClientVerifier (struct)
macro_rules! Depcrate_x509_verifyPyClientVerifier {
() => {
// Module: crate::x509::verify
// Provides: {"PyClientVerifier"}
// Dependencies: {}
# [pyo3 :: pyclass (frozen , name = "ClientVerifier" , module = "cryptography.hazmat.bindings._rust.x509")] pub (crate) struct PyClientVerifier { # [pyo3 (get , name = "policy")] py_policy : pyo3 :: Py < PyPolicy > , # [pyo3 (get)] store : pyo3 :: Py < PyStore > , }
};
}
