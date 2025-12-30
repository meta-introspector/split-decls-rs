// Generated macro for PyServerVerifier (struct)
macro_rules! Depcrate_x509_verifyPyServerVerifier {
() => {
// Module: crate::x509::verify
// Provides: {"PyServerVerifier"}
// Dependencies: {}
# [pyo3 :: pyclass (frozen , name = "ServerVerifier" , module = "cryptography.hazmat.bindings._rust.x509")] pub (crate) struct PyServerVerifier { # [pyo3 (get , name = "policy")] py_policy : pyo3 :: Py < PyPolicy > , # [pyo3 (get)] store : pyo3 :: Py < PyStore > , }
};
}
