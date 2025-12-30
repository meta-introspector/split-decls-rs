// Generated macro for ECPrivateKey (struct)
macro_rules! Depcrate_backend_ecECPrivateKey {
() => {
// Module: crate::backend::ec
// Provides: {"ECPrivateKey"}
// Dependencies: {}
# [pyo3 :: pyclass (frozen , module = "cryptography.hazmat.bindings._rust.openssl.ec")] pub (crate) struct ECPrivateKey { pkey : openssl :: pkey :: PKey < openssl :: pkey :: Private > , # [pyo3 (get)] curve : pyo3 :: Py < pyo3 :: PyAny > , }
};
}
