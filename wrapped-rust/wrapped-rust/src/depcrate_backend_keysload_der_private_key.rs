// Generated macro for load_der_private_key (function)
macro_rules! Depcrate_backend_keysload_der_private_key {
() => {
// Module: crate::backend::keys
// Provides: {"load_der_private_key"}
// Dependencies: {}
# [pyo3 :: pyfunction] # [pyo3 (signature = (data , password , backend = None , *, unsafe_skip_rsa_key_validation = false))] fn load_der_private_key < 'p > (py : pyo3 :: Python < 'p > , data : CffiBuf < '_ > , password : Option < CffiBuf < '_ > > , backend : Option < pyo3 :: Bound < '_ , pyo3 :: PyAny > > , unsafe_skip_rsa_key_validation : bool ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let _ = backend ; load_der_private_key_bytes (py , data . as_bytes () , password . as_ref () . map (| v | v . as_bytes ()) , unsafe_skip_rsa_key_validation ,) }
};
}
