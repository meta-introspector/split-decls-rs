// Generated macro for BaseArgon2 (struct)
macro_rules! Depcrate_backend_kdfBaseArgon2 {
() => {
// Module: crate::backend::kdf
// Provides: {"BaseArgon2"}
// Dependencies: {}
struct BaseArgon2 { # [cfg (CRYPTOGRAPHY_OPENSSL_320_OR_GREATER)] salt : pyo3 :: Py < pyo3 :: types :: PyBytes > , # [cfg (CRYPTOGRAPHY_OPENSSL_320_OR_GREATER)] length : usize , # [cfg (CRYPTOGRAPHY_OPENSSL_320_OR_GREATER)] iterations : u32 , # [cfg (CRYPTOGRAPHY_OPENSSL_320_OR_GREATER)] lanes : u32 , # [cfg (CRYPTOGRAPHY_OPENSSL_320_OR_GREATER)] memory_cost : u32 , # [cfg (CRYPTOGRAPHY_OPENSSL_320_OR_GREATER)] ad : Option < pyo3 :: Py < pyo3 :: types :: PyBytes > > , # [cfg (CRYPTOGRAPHY_OPENSSL_320_OR_GREATER)] secret : Option < pyo3 :: Py < pyo3 :: types :: PyBytes > > , # [cfg (CRYPTOGRAPHY_OPENSSL_320_OR_GREATER)] used : bool , }
};
}
