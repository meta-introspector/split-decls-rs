// Generated macro for XOFHash (struct)
macro_rules! Depcrate_backend_hashesXOFHash {
() => {
// Module: crate::backend::hashes
// Provides: {"XOFHash"}
// Dependencies: {}
# [pyo3 :: pyclass (module = "cryptography.hazmat.bindings._rust.openssl.hashes")] pub (crate) struct XOFHash { # [pyo3 (get)] algorithm : pyo3 :: Py < pyo3 :: PyAny > , ctx : openssl :: hash :: Hasher , bytes_remaining : u64 , squeezed : bool , }
};
}
