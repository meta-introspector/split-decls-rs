// Generated macro for Pbkdf2Hmac (struct)
macro_rules! Depcrate_backend_kdfPbkdf2Hmac {
() => {
// Module: crate::backend::kdf
// Provides: {"Pbkdf2Hmac"}
// Dependencies: {}
# [pyo3 :: pyclass (module = "cryptography.hazmat.primitives.kdf.pbkdf2" , name = "PBKDF2HMAC")] struct Pbkdf2Hmac { md : openssl :: hash :: MessageDigest , salt : pyo3 :: Py < pyo3 :: types :: PyBytes > , iterations : usize , length : usize , used : bool , }
};
}
