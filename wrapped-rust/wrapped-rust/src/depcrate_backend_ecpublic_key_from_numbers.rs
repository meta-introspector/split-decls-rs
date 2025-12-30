// Generated macro for public_key_from_numbers (function)
macro_rules! Depcrate_backend_ecpublic_key_from_numbers {
() => {
// Module: crate::backend::ec
// Provides: {"public_key_from_numbers"}
// Dependencies: {}
fn public_key_from_numbers (py : pyo3 :: Python < '_ > , numbers : & EllipticCurvePublicNumbers , curve : & openssl :: ec :: EcGroupRef ,) -> CryptographyResult < openssl :: ec :: EcKey < openssl :: pkey :: Public > > { if numbers . x . bind (py) . lt (0) ? || numbers . y . bind (py) . lt (0) ? { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("Invalid EC key. Both x and y must be non-negative." ,) ,)) ; } let x = utils :: py_int_to_bn (py , numbers . x . bind (py)) ? ; let y = utils :: py_int_to_bn (py , numbers . y . bind (py)) ? ; let mut point = openssl :: ec :: EcPoint :: new (curve) ? ; let mut bn_ctx = openssl :: bn :: BigNumContext :: new () ? ; point . set_affine_coordinates_gfp (curve , & x , & y , & mut bn_ctx) . map_err (| _ | { pyo3 :: exceptions :: PyValueError :: new_err ("Invalid EC key. Point is not on the curve specified." ,) }) ? ; Ok (openssl :: ec :: EcKey :: from_public_key (curve , & point) ?) }
};
}
