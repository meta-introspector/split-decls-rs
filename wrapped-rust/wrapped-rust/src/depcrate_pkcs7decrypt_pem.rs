// Generated macro for decrypt_pem (function)
macro_rules! Depcrate_pkcs7decrypt_pem {
() => {
// Module: crate::pkcs7
// Provides: {"decrypt_pem"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn decrypt_pem < 'p > (py : pyo3 :: Python < 'p > , data : & [u8] , certificate : pyo3 :: Bound < 'p , x509 :: certificate :: Certificate > , private_key : pyo3 :: Bound < 'p , pyo3 :: types :: PyAny > , options : & pyo3 :: Bound < 'p , pyo3 :: types :: PyList > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: types :: PyBytes > > { let pem_str = std :: str :: from_utf8 (data) . map_err (| _ | pyo3 :: exceptions :: PyValueError :: new_err ("Invalid PEM data")) ? ; let pem = pem :: parse (pem_str) . map_err (| _ | pyo3 :: exceptions :: PyValueError :: new_err ("Failed to parse PEM data")) ? ; if pem . tag () != "PKCS7" { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("The provided PEM data does not have the PKCS7 tag." ,) ,)) ; } decrypt_der (py , & pem . into_contents () , certificate , private_key , options) }
};
}
