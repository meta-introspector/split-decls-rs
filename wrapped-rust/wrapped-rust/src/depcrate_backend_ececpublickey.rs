// Generated macro for ECPublicKey (struct)
macro_rules! Depcrate_backend_ecECPublicKey {
() => {
// Module: crate::backend::ec
// Provides: {"ECPublicKey"}
// Dependencies: {}
# [pyo3 :: pyclass (frozen , module = "cryptography.hazmat.bindings._rust.openssl.ec")] pub (crate) struct ECPublicKey { pkey : openssl :: pkey :: PKey < openssl :: pkey :: Public > , # [pyo3 (get)] curve : pyo3 :: Py < pyo3 :: PyAny > , }
};
}
