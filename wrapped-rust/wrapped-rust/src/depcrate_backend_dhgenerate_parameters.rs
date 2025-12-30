// Generated macro for generate_parameters (function)
macro_rules! Depcrate_backend_dhgenerate_parameters {
() => {
// Module: crate::backend::dh
// Provides: {"generate_parameters"}
// Dependencies: {}
# [pyo3 :: pyfunction] # [pyo3 (signature = (generator , key_size , backend = None))] fn generate_parameters (generator : u32 , key_size : u32 , backend : Option < pyo3 :: Bound < '_ , pyo3 :: PyAny > > ,) -> CryptographyResult < DHParameters > { let _ = backend ; if key_size < cryptography_key_parsing :: MIN_DH_MODULUS_SIZE { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err (format ! ("DH key_size must be at least {} bits" , cryptography_key_parsing :: MIN_DH_MODULUS_SIZE)) ,)) ; } if generator != 2 && generator != 5 { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("DH generator must be 2 or 5") ,)) ; } let dh = openssl :: dh :: Dh :: generate_params (key_size , generator) . map_err (| _ | pyo3 :: exceptions :: PyValueError :: new_err ("Unable to generate DH parameters")) ? ; Ok (DHParameters { dh }) }
};
}
