// Generated macro for check_dsa_parameters (function)
macro_rules! Depcrate_backend_dsacheck_dsa_parameters {
() => {
// Module: crate::backend::dsa
// Provides: {"check_dsa_parameters"}
// Dependencies: {}
fn check_dsa_parameters (py : pyo3 :: Python < '_ > , parameters : & DsaParameterNumbers ,) -> CryptographyResult < () > { if ! [1024 , 2048 , 3072 , 4096] . contains (& parameters . p . bind (py) . call_method0 ("bit_length") ? . extract :: < usize > () ? ,) { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("p must be exactly 1024, 2048, 3072, or 4096 bits long" ,) ,)) ; } if ! [160 , 224 , 256] . contains (& parameters . q . bind (py) . call_method0 ("bit_length") ? . extract :: < usize > () ? ,) { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("q must be exactly 160, 224, or 256 bits long") ,)) ; } if parameters . g . bind (py) . le (1) ? || parameters . g . bind (py) . ge (parameters . p . bind (py)) ? { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("g, p don't satisfy 1 < g < p.") ,)) ; } Ok (()) }
};
}
