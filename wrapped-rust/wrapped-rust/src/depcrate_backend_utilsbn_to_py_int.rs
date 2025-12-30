// Generated macro for bn_to_py_int (function)
macro_rules! Depcrate_backend_utilsbn_to_py_int {
() => {
// Module: crate::backend::utils
// Provides: {"bn_to_py_int"}
// Dependencies: {}
pub (crate) fn bn_to_py_int < 'p > (py : pyo3 :: Python < 'p > , b : & openssl :: bn :: BigNumRef ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { assert ! (! b . is_negative ()) ; let int_type = py . get_type :: < pyo3 :: types :: PyInt > () ; Ok (int_type . call_method1 (pyo3 :: intern ! (py , "from_bytes") , (b . to_vec () , pyo3 :: intern ! (py , "big")) ,) ?) }
};
}
