// Generated macro for from_public_bytes (function)
macro_rules! Depcrate_backend_ecfrom_public_bytes {
() => {
// Module: crate::backend::ec
// Provides: {"from_public_bytes"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn from_public_bytes (py : pyo3 :: Python < '_ > , py_curve : pyo3 :: Bound < '_ , pyo3 :: PyAny > , data : & [u8] ,) -> CryptographyResult < ECPublicKey > { let curve = curve_from_py_curve (py , py_curve . clone ()) ? ; let mut bn_ctx = openssl :: bn :: BigNumContext :: new () ? ; let point = openssl :: ec :: EcPoint :: from_bytes (& curve , data , & mut bn_ctx) . map_err (| _ | pyo3 :: exceptions :: PyValueError :: new_err ("Invalid EC key.")) ? ; let ec = openssl :: ec :: EcKey :: from_public_key (& curve , & point) ? ; let pkey = openssl :: pkey :: PKey :: from_ec_key (ec) ? ; Ok (ECPublicKey { pkey , curve : py_curve . into () , }) }
};
}
