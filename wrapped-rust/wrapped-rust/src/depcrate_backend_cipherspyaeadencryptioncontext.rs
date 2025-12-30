// Generated macro for PyAEADEncryptionContext (struct)
macro_rules! Depcrate_backend_ciphersPyAEADEncryptionContext {
() => {
// Module: crate::backend::ciphers
// Provides: {"PyAEADEncryptionContext"}
// Dependencies: {}
# [pyo3 :: pyclass (module = "cryptography.hazmat.bindings._rust.openssl.ciphers" , name = "AEADEncryptionContext")] struct PyAEADEncryptionContext { ctx : Option < CipherContext > , tag : Option < pyo3 :: Py < pyo3 :: types :: PyBytes > > , updated : bool , bytes_remaining : u64 , aad_bytes_remaining : u64 , }
};
}
