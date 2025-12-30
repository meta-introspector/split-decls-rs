// Generated macro for check_public_key_components (function)
macro_rules! Depcrate_backend_rsacheck_public_key_components {
() => {
// Module: crate::backend::rsa
// Provides: {"check_public_key_components"}
// Dependencies: {}
fn check_public_key_components (e : & pyo3 :: Bound < '_ , pyo3 :: types :: PyInt > , n : & pyo3 :: Bound < '_ , pyo3 :: types :: PyInt > ,) -> CryptographyResult < () > { if n . lt (3) ? { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("n must be >= 3.") ,)) ; } if e . lt (3) ? || e . ge (n) ? { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("e must be >= 3 and < n.") ,)) ; } if e . bitand (1) ? . eq (0) ? { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("e must be odd.") ,)) ; } Ok (()) }
};
}
