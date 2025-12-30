// Generated macro for public_key_from_pkey (function)
macro_rules! Depcrate_backend_ecpublic_key_from_pkey {
() => {
// Module: crate::backend::ec
// Provides: {"public_key_from_pkey"}
// Dependencies: {}
pub (crate) fn public_key_from_pkey (py : pyo3 :: Python < '_ > , pkey : & openssl :: pkey :: PKeyRef < openssl :: pkey :: Public > ,) -> CryptographyResult < ECPublicKey > { let ec = pkey . ec_key () ? ; let curve = py_curve_from_curve (py , ec . group ()) ? ; check_key_infinity (& ec) ? ; Ok (ECPublicKey { pkey : pkey . to_owned () , curve : curve . into () , }) }
};
}
