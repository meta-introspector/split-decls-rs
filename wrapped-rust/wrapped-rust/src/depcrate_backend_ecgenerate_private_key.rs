// Generated macro for generate_private_key (function)
macro_rules! Depcrate_backend_ecgenerate_private_key {
() => {
// Module: crate::backend::ec
// Provides: {"generate_private_key"}
// Dependencies: {}
# [pyo3 :: pyfunction] # [pyo3 (signature = (curve , backend = None))] fn generate_private_key (py : pyo3 :: Python < '_ > , curve : pyo3 :: Bound < '_ , pyo3 :: PyAny > , backend : Option < pyo3 :: Bound < '_ , pyo3 :: PyAny > > ,) -> CryptographyResult < ECPrivateKey > { let _ = backend ; let ossl_curve = curve_from_py_curve (py , curve) ? ; let key = openssl :: ec :: EcKey :: generate (& ossl_curve) ? ; Ok (ECPrivateKey { pkey : openssl :: pkey :: PKey :: from_ec_key (key) ? , curve : py_curve_from_curve (py , & ossl_curve) ? . into () , }) }
};
}
