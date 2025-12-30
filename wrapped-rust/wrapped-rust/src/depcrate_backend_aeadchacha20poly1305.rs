// Generated macro for ChaCha20Poly1305 (struct)
macro_rules! Depcrate_backend_aeadChaCha20Poly1305 {
() => {
// Module: crate::backend::aead
// Provides: {"ChaCha20Poly1305"}
// Dependencies: {}
# [pyo3 :: pyclass (frozen , module = "cryptography.hazmat.bindings._rust.openssl.aead")] struct ChaCha20Poly1305 { # [cfg (any (CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC))] ctx : EvpAead , # [cfg (any (CRYPTOGRAPHY_OPENSSL_320_OR_GREATER , CRYPTOGRAPHY_IS_LIBRESSL))] ctx : EvpCipherAead , # [cfg (not (any (CRYPTOGRAPHY_IS_LIBRESSL , CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC , CRYPTOGRAPHY_OPENSSL_320_OR_GREATER)))] ctx : LazyEvpCipherAead , }
};
}
