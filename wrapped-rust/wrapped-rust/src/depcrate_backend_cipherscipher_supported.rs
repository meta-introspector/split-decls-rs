// Generated macro for cipher_supported (function)
macro_rules! Depcrate_backend_cipherscipher_supported {
() => {
// Module: crate::backend::ciphers
// Provides: {"cipher_supported"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn cipher_supported (py : pyo3 :: Python < '_ > , algorithm : pyo3 :: Bound < '_ , pyo3 :: PyAny > , mode : pyo3 :: Bound < '_ , pyo3 :: PyAny > ,) -> CryptographyResult < bool > { Ok (cipher_registry :: get_cipher (py , algorithm , mode . get_type () . into_any ()) ? . is_some ()) }
};
}
