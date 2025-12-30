// Generated macro for AesCcm (struct)
macro_rules! Depcrate_backend_aeadAesCcm {
() => {
// Module: crate::backend::aead
// Provides: {"AesCcm"}
// Dependencies: {}
# [pyo3 :: pyclass (frozen , module = "cryptography.hazmat.bindings._rust.openssl.aead" , name = "AESCCM")] struct AesCcm { ctx : LazyEvpCipherAead , tag_length : usize , }
};
}
