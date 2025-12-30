// Generated macro for derive_private_key (function)
macro_rules! Depcrate_backend_ecderive_private_key {
() => {
// Module: crate::backend::ec
// Provides: {"derive_private_key"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn derive_private_key (py : pyo3 :: Python < '_ > , py_private_value : & pyo3 :: Bound < '_ , pyo3 :: types :: PyInt > , py_curve : pyo3 :: Bound < '_ , pyo3 :: PyAny > ,) -> CryptographyResult < ECPrivateKey > { let curve = curve_from_py_curve (py , py_curve . clone ()) ? ; let private_value = utils :: py_int_to_bn (py , py_private_value) ? ; let mut point = openssl :: ec :: EcPoint :: new (& curve) ? ; let bn_ctx = openssl :: bn :: BigNumContext :: new () ? ; point . mul_generator (& curve , & private_value , & bn_ctx) ? ; let ec = openssl :: ec :: EcKey :: from_private_components (& curve , & private_value , & point) . map_err (| _ | pyo3 :: exceptions :: PyValueError :: new_err ("Invalid EC key")) ? ; ec . check_key () . map_err (| _ | { pyo3 :: exceptions :: PyValueError :: new_err ("Invalid EC key (key out of range, infinity, etc.)") }) ? ; let pkey = openssl :: pkey :: PKey :: from_ec_key (ec) ? ; Ok (ECPrivateKey { pkey , curve : py_curve . into () , }) }
};
}
