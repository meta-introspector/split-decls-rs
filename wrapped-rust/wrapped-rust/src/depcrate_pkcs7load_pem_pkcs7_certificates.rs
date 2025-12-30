// Generated macro for load_pem_pkcs7_certificates (function)
macro_rules! Depcrate_pkcs7load_pem_pkcs7_certificates {
() => {
// Module: crate::pkcs7
// Provides: {"load_pem_pkcs7_certificates"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn load_pem_pkcs7_certificates (py : pyo3 :: Python < '_ > , data : pyo3 :: Py < pyo3 :: types :: PyBytes > ,) -> CryptographyResult < pyo3 :: Bound < '_ , pyo3 :: types :: PyList > > { let pem_block = pem :: parse (data . as_bytes (py)) ? ; if pem_block . tag () != "PKCS7" { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("The provided PEM data does not have the PKCS7 tag." ,) ,)) ; } let data = pyo3 :: types :: PyBytes :: new (py , pem_block . contents ()) . unbind () ; load_der_pkcs7_certificates (py , data) }
};
}
