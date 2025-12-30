// Generated macro for load_pkcs7_certificates (function)
macro_rules! Depcrate_pkcs7load_pkcs7_certificates {
() => {
// Module: crate::pkcs7
// Provides: {"load_pkcs7_certificates"}
// Dependencies: {}
# [cfg (not (any (CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC)))] fn load_pkcs7_certificates (py : pyo3 :: Python < '_ > , pkcs7 : Pkcs7 ,) -> CryptographyResult < pyo3 :: Bound < '_ , pyo3 :: types :: PyList > > { let nid = pkcs7 . type_ () . map (| t | t . nid ()) ; if nid != Some (openssl :: nid :: Nid :: PKCS7_SIGNED) { let nid_string = nid . map_or ("empty" . to_string () , | n | n . as_raw () . to_string ()) ; return Err (CryptographyError :: from (exceptions :: UnsupportedAlgorithm :: new_err ((format ! ("Only basic signed structures are currently supported. NID for this data was {nid_string}") , exceptions :: Reasons :: UNSUPPORTED_SERIALIZATION ,)) ,)) ; } let signed_certificates = pkcs7 . signed () . and_then (| x | x . certificates ()) ; match signed_certificates { None => Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("The provided PKCS7 has no certificate data, but a cert loading method was called." ,) ,)) , Some (certificates) => { let result = pyo3 :: types :: PyList :: empty (py) ; for c in certificates { let cert_der = pyo3 :: types :: PyBytes :: new (py , c . to_der () ? . as_slice ()) . unbind () ; let cert = load_der_x509_certificate (py , cert_der , None) ? ; result . append (cert) ? ; } Ok (result) } } }
};
}
