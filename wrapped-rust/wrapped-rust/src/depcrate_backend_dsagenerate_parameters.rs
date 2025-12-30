// Generated macro for generate_parameters (function)
macro_rules! Depcrate_backend_dsagenerate_parameters {
() => {
// Module: crate::backend::dsa
// Provides: {"generate_parameters"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn generate_parameters (key_size : u32) -> CryptographyResult < DsaParameters > { let dsa = openssl :: dsa :: Dsa :: generate_params (key_size) ? ; Ok (DsaParameters { dsa }) }
};
}
