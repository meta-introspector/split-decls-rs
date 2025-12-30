// Generated macro for serialize_java_truststore (function)
macro_rules! Depcrate_pkcs12serialize_java_truststore {
() => {
// Module: crate::pkcs12
// Provides: {"serialize_java_truststore"}
// Dependencies: {}
# [pyo3 :: pyfunction] # [pyo3 (signature = (pkcs12_certs , encryption_algorithm))] fn serialize_java_truststore < 'p > (py : pyo3 :: Python < 'p > , pkcs12_certs : Vec < pyo3 :: Py < PKCS12Certificate > > , encryption_algorithm : pyo3 :: Bound < '_ , pyo3 :: PyAny > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: types :: PyBytes > > { let encryption_details = decode_encryption_algorithm (py , encryption_algorithm) ? ; let mut safebags = vec ! [] ; for cert in & pkcs12_certs { safebags . push (cert_to_bag (cert . get () . certificate . get () , cert . get () . friendly_name . as_ref () . map (| v | v . as_bytes (py)) , None , true ,) ?) ; } serialize_safebags (py , & safebags , & encryption_details) }
};
}
