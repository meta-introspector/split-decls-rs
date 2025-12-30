// Generated macro for check_dsa_private_numbers (function)
macro_rules! Depcrate_backend_dsacheck_dsa_private_numbers {
() => {
// Module: crate::backend::dsa
// Provides: {"check_dsa_private_numbers"}
// Dependencies: {}
fn check_dsa_private_numbers (py : pyo3 :: Python < '_ > , numbers : & DsaPrivateNumbers ,) -> CryptographyResult < () > { let params = numbers . public_numbers . get () . parameter_numbers . get () ; check_dsa_parameters (py , params) ? ; if numbers . x . bind (py) . le (0) ? || numbers . x . bind (py) . ge (params . q . bind (py)) ? { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("x must be > 0 and < q.") ,)) ; } if (* * numbers . public_numbers . get () . y . bind (py)) . ne (params . g . bind (py) . pow (numbers . x . bind (py) , Some (params . p . bind (py))) ?) ? { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("y must be equal to (g ** x % p).") ,)) ; } Ok (()) }
};
}
