// Generated macro for impl_142 (impl)
macro_rules! Depcrate_backend_dsaimpl_142 {
() => {
// Module: crate::backend::dsa
// Provides: {"impl_142"}
// Dependencies: {}
# [pyo3 :: pymethods] impl DsaParameters { fn generate_private_key (& self) -> CryptographyResult < DsaPrivateKey > { let dsa = clone_dsa_params (& self . dsa) ? . generate_key () ? ; let pkey = openssl :: pkey :: PKey :: from_dsa (dsa) ? ; Ok (DsaPrivateKey { pkey }) } fn parameter_numbers (& self , py : pyo3 :: Python < '_ >) -> CryptographyResult < DsaParameterNumbers > { let py_p = utils :: bn_to_py_int (py , self . dsa . p ()) ? ; let py_q = utils :: bn_to_py_int (py , self . dsa . q ()) ? ; let py_g = utils :: bn_to_py_int (py , self . dsa . g ()) ? ; Ok (DsaParameterNumbers { p : py_p . extract () ? , q : py_q . extract () ? , g : py_g . extract () ? , }) } }
};
}
