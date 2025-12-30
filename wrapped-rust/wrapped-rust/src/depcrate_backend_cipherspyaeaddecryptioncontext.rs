// Generated macro for PyAEADDecryptionContext (struct)
macro_rules! Depcrate_backend_ciphersPyAEADDecryptionContext {
() => {
// Module: crate::backend::ciphers
// Provides: {"PyAEADDecryptionContext"}
// Dependencies: {}
# [pyo3 :: pyclass (module = "cryptography.hazmat.bindings._rust.openssl.ciphers" , name = "AEADDecryptionContext")] struct PyAEADDecryptionContext { ctx : Option < CipherContext > , updated : bool , bytes_remaining : u64 , aad_bytes_remaining : u64 , }
};
}
