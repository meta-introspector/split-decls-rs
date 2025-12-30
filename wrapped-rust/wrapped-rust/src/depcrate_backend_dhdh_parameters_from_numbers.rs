// Generated macro for dh_parameters_from_numbers (function)
macro_rules! Depcrate_backend_dhdh_parameters_from_numbers {
() => {
// Module: crate::backend::dh
// Provides: {"dh_parameters_from_numbers"}
// Dependencies: {}
fn dh_parameters_from_numbers (py : pyo3 :: Python < '_ > , numbers : & DHParameterNumbers ,) -> CryptographyResult < openssl :: dh :: Dh < openssl :: pkey :: Params > > { let p = utils :: py_int_to_bn (py , numbers . p . bind (py)) ? ; let q = numbers . q . as_ref () . map (| v | utils :: py_int_to_bn (py , v . bind (py))) . transpose () ? ; let g = utils :: py_int_to_bn (py , numbers . g . bind (py)) ? ; let dh = openssl :: dh :: Dh :: from_pqg (p , q , g) ? ; if ! dh . check_key () ? { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("Invalid DH parameters") ,)) ; } Ok (dh) }
};
}
