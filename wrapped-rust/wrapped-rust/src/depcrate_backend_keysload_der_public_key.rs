// Generated macro for load_der_public_key (function)
macro_rules! Depcrate_backend_keysload_der_public_key {
() => {
// Module: crate::backend::keys
// Provides: {"load_der_public_key"}
// Dependencies: {}
# [pyo3 :: pyfunction] # [pyo3 (signature = (data , backend = None))] fn load_der_public_key < 'p > (py : pyo3 :: Python < 'p > , data : CffiBuf < '_ > , backend : Option < pyo3 :: Bound < '_ , pyo3 :: PyAny > > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let _ = backend ; load_der_public_key_bytes (py , data . as_bytes ()) }
};
}
