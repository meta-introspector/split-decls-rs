// Generated macro for py_int_to_bn (function)
macro_rules! Depcrate_backend_utilspy_int_to_bn {
() => {
// Module: crate::backend::utils
// Provides: {"py_int_to_bn"}
// Dependencies: {}
pub (crate) fn py_int_to_bn (py : pyo3 :: Python < '_ > , v : & pyo3 :: Bound < '_ , pyo3 :: PyAny > ,) -> CryptographyResult < openssl :: bn :: BigNum > { let n = v . call_method0 (pyo3 :: intern ! (py , "bit_length")) ? . extract :: < usize > () ? / 8 + 1 ; let bytes = v . call_method1 (pyo3 :: intern ! (py , "to_bytes") , (n , pyo3 :: intern ! (py , "big"))) ? . extract :: < pyo3 :: pybacked :: PyBackedBytes > () ? ; Ok (openssl :: bn :: BigNum :: from_slice (& bytes) ?) }
};
}
