// Generated macro for load_pem_public_key (function)
macro_rules! Depcrate_backend_keysload_pem_public_key {
() => {
// Module: crate::backend::keys
// Provides: {"load_pem_public_key"}
// Dependencies: {}
# [pyo3 :: pyfunction] # [pyo3 (signature = (data , backend = None))] fn load_pem_public_key < 'p > (py : pyo3 :: Python < 'p > , data : CffiBuf < '_ > , backend : Option < pyo3 :: Bound < '_ , pyo3 :: PyAny > > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let _ = backend ; let p = pem :: parse (data . as_bytes ()) ? ; let pkey = match p . tag () { "RSA PUBLIC KEY" => { match cryptography_key_parsing :: rsa :: parse_pkcs1_public_key (p . contents ()) { Ok (pkey) => pkey , Err (err) => { let pkey = cryptography_key_parsing :: spki :: parse_public_key (p . contents ()) . map_err (| _ | err) ? ; if pkey . id () != openssl :: pkey :: Id :: RSA { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("Incorrect PEM delimiter for key type." ,) ,)) ; } pkey } } } "PUBLIC KEY" => cryptography_key_parsing :: spki :: parse_public_key (p . contents ()) ? , _ => return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("Valid PEM but no BEGIN PUBLIC KEY/END PUBLIC KEY delimiters. Are you sure this is a public key?"))) , } ; public_key_from_pkey (py , & pkey , pkey . id ()) }
};
}
