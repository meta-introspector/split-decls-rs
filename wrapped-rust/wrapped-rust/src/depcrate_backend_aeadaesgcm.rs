// Generated macro for AesGcm (struct)
macro_rules! Depcrate_backend_aeadAesGcm {
() => {
// Module: crate::backend::aead
// Provides: {"AesGcm"}
// Dependencies: {}
# [pyo3 :: pyclass (frozen , module = "cryptography.hazmat.bindings._rust.openssl.aead" , name = "AESGCM")] struct AesGcm { # [cfg (any (CRYPTOGRAPHY_OPENSSL_320_OR_GREATER , CRYPTOGRAPHY_IS_LIBRESSL , CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC))] ctx : EvpCipherAead , # [cfg (not (any (CRYPTOGRAPHY_OPENSSL_320_OR_GREATER , CRYPTOGRAPHY_IS_LIBRESSL , CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC)))] ctx : LazyEvpCipherAead , }
};
}
