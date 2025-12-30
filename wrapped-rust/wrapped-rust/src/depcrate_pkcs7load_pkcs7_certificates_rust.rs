// Generated macro for load_pkcs7_certificates_rust (function)
macro_rules! Depcrate_pkcs7load_pkcs7_certificates_rust {
() => {
// Module: crate::pkcs7
// Provides: {"load_pkcs7_certificates_rust"}
// Dependencies: {}
fn load_pkcs7_certificates_rust (py : pyo3 :: Python < '_ > , data : pyo3 :: Py < pyo3 :: types :: PyBytes > ,) -> CryptographyResult < pyo3 :: Bound < '_ , pyo3 :: types :: PyList > > { try_list_of_certificates (py , data , | data , cb | { let p7 = asn1 :: parse_single :: < pkcs7 :: ContentInfo < '_ > > (data . as_bytes (py)) ? ; let pkcs7 :: Content :: SignedData (signed_data) = p7 . content else { return Err (CryptographyError :: from (exceptions :: UnsupportedAlgorithm :: new_err (("Only basic signed structures are currently supported." , exceptions :: Reasons :: UNSUPPORTED_SERIALIZATION ,)) ,)) ; } ; let Some (certs) = signed_data . into_inner () . certificates else { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("The provided PKCS7 has no certificate data, but a cert loading method was called." ,) ,)) ; } ; for c in certs . unwrap_read () . clone () { cb (c) ? ; } Ok (()) }) }
};
}
