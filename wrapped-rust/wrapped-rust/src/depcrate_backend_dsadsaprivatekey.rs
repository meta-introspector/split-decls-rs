// Generated macro for DsaPrivateKey (struct)
macro_rules! Depcrate_backend_dsaDsaPrivateKey {
() => {
// Module: crate::backend::dsa
// Provides: {"DsaPrivateKey"}
// Dependencies: {}
# [pyo3 :: pyclass (frozen , module = "cryptography.hazmat.bindings._rust.openssl.dsa" , name = "DSAPrivateKey")] pub (crate) struct DsaPrivateKey { pkey : openssl :: pkey :: PKey < openssl :: pkey :: Private > , }
};
}
