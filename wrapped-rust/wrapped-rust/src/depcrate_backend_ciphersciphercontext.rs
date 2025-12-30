// Generated macro for CipherContext (struct)
macro_rules! Depcrate_backend_ciphersCipherContext {
() => {
// Module: crate::backend::ciphers
// Provides: {"CipherContext"}
// Dependencies: {}
pub (crate) struct CipherContext { ctx : openssl :: cipher_ctx :: CipherCtx , py_mode : pyo3 :: Py < pyo3 :: PyAny > , py_algorithm : pyo3 :: Py < pyo3 :: PyAny > , side : openssl :: symm :: Mode , }
};
}
