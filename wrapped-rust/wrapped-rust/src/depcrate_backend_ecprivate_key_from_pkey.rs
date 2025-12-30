// Generated macro for private_key_from_pkey (function)
macro_rules! Depcrate_backend_ecprivate_key_from_pkey {
() => {
// Module: crate::backend::ec
// Provides: {"private_key_from_pkey"}
// Dependencies: {}
pub (crate) fn private_key_from_pkey (py : pyo3 :: Python < '_ > , pkey : & openssl :: pkey :: PKeyRef < openssl :: pkey :: Private > ,) -> CryptographyResult < ECPrivateKey > { let ec_key = pkey . ec_key () . map_err (| _ | pyo3 :: exceptions :: PyValueError :: new_err ("Invalid EC key")) ? ; let curve = py_curve_from_curve (py , ec_key . group ()) ? ; check_key_infinity (& ec_key) ? ; Ok (ECPrivateKey { pkey : pkey . to_owned () , curve : curve . into () , }) }
};
}
