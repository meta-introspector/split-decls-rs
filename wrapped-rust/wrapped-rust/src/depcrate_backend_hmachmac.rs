// Generated macro for Hmac (struct)
macro_rules! Depcrate_backend_hmacHmac {
() => {
// Module: crate::backend::hmac
// Provides: {"Hmac"}
// Dependencies: {}
# [pyo3 :: pyclass (module = "cryptography.hazmat.bindings._rust.openssl.hmac" , name = "HMAC")] pub (crate) struct Hmac { # [pyo3 (get)] algorithm : pyo3 :: Py < pyo3 :: PyAny > , ctx : Option < cryptography_openssl :: hmac :: Hmac > , }
};
}
