// Generated macro for PyVerifiedClient (struct)
macro_rules! Depcrate_x509_verifyPyVerifiedClient {
() => {
// Module: crate::x509::verify
// Provides: {"PyVerifiedClient"}
// Dependencies: {}
# [pyo3 :: pyclass (frozen , name = "VerifiedClient" , module = "cryptography.hazmat.bindings._rust.x509")] pub (crate) struct PyVerifiedClient { # [pyo3 (get)] subjects : Option < pyo3 :: Py < pyo3 :: PyAny > > , # [pyo3 (get)] chain : pyo3 :: Py < pyo3 :: types :: PyList > , }
};
}
