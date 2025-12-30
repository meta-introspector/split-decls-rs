// Generated macro for Scrypt (struct)
macro_rules! Depcrate_backend_kdfScrypt {
() => {
// Module: crate::backend::kdf
// Provides: {"Scrypt"}
// Dependencies: {}
# [pyo3 :: pyclass (module = "cryptography.hazmat.primitives.kdf.scrypt")] struct Scrypt { # [cfg (not (CRYPTOGRAPHY_IS_LIBRESSL))] salt : pyo3 :: Py < pyo3 :: types :: PyBytes > , # [cfg (not (CRYPTOGRAPHY_IS_LIBRESSL))] length : usize , # [cfg (not (CRYPTOGRAPHY_IS_LIBRESSL))] n : u64 , # [cfg (not (CRYPTOGRAPHY_IS_LIBRESSL))] r : u64 , # [cfg (not (CRYPTOGRAPHY_IS_LIBRESSL))] p : u64 , # [cfg (not (CRYPTOGRAPHY_IS_LIBRESSL))] used : bool , }
};
}
