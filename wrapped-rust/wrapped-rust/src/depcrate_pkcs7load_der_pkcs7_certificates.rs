// Generated macro for load_der_pkcs7_certificates (function)
macro_rules! Depcrate_pkcs7load_der_pkcs7_certificates {
() => {
// Module: crate::pkcs7
// Provides: {"load_der_pkcs7_certificates"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn load_der_pkcs7_certificates (py : pyo3 :: Python < '_ > , data : pyo3 :: Py < pyo3 :: types :: PyBytes > ,) -> CryptographyResult < pyo3 :: Bound < '_ , pyo3 :: types :: PyList > > { cfg_if :: cfg_if ! { if # [cfg (not (any (CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC)))] { let pkcs7_decoded = openssl :: pkcs7 :: Pkcs7 :: from_der (data . as_bytes (py)) . map_err (| _ | { CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("Unable to parse PKCS7 data" ,)) }) ?; let result = load_pkcs7_certificates (py , pkcs7_decoded) ?; if let Err (e) = load_pkcs7_certificates_rust (py , data) { let err = pyo3 :: PyErr :: from (e) ; let warning_cls = pyo3 :: exceptions :: PyUserWarning :: type_object (py) ; let message = CString :: new (format ! ("PKCS#7 certificates could not be parsed as DER, falling back to parsing as BER. Please file an issue at https://github.com/pyca/cryptography/issues explaining how your PKCS#7 certificates were created. In the future, this may become an exception. Error details: {err}")) . unwrap () ; pyo3 :: PyErr :: warn (py , & warning_cls , & message , 1) ?; } Ok (result) } else { load_pkcs7_certificates_rust (py , data) } } }
};
}
