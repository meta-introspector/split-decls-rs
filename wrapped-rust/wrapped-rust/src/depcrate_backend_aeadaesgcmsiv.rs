// Generated macro for AesGcmSiv (struct)
macro_rules! Depcrate_backend_aeadAesGcmSiv {
() => {
// Module: crate::backend::aead
// Provides: {"AesGcmSiv"}
// Dependencies: {}
# [pyo3 :: pyclass (frozen , module = "cryptography.hazmat.bindings._rust.openssl.aead" , name = "AESGCMSIV")] struct AesGcmSiv { # [cfg (any (CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC))] ctx : EvpAead , # [cfg (not (any (CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC)))] ctx : EvpCipherAead , }
};
}
