// Generated macro for load_der_public_key_bytes (function)
macro_rules! Depcrate_backend_keysload_der_public_key_bytes {
() => {
// Module: crate::backend::keys
// Provides: {"load_der_public_key_bytes"}
// Dependencies: {}
pub (crate) fn load_der_public_key_bytes < 'p > (py : pyo3 :: Python < 'p > , data : & [u8] ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { match cryptography_key_parsing :: spki :: parse_public_key (data) { Ok (pkey) => public_key_from_pkey (py , & pkey , pkey . id ()) , Err (e) => { let pkey = cryptography_key_parsing :: rsa :: parse_pkcs1_public_key (data) . map_err (| _ | e) ? ; public_key_from_pkey (py , & pkey , pkey . id ()) } } }
};
}
