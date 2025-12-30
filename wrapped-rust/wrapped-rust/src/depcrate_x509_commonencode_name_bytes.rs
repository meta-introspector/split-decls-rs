// Generated macro for encode_name_bytes (function)
macro_rules! Depcrate_x509_commonencode_name_bytes {
() => {
// Module: crate::x509::common
// Provides: {"encode_name_bytes"}
// Dependencies: {}
# [pyo3 :: pyfunction] pub (crate) fn encode_name_bytes < 'p > (py : pyo3 :: Python < 'p > , py_name : & pyo3 :: Bound < 'p , pyo3 :: PyAny > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: types :: PyBytes > > { let ka = cryptography_keepalive :: KeepAlive :: new () ; let name = encode_name (py , & ka , py_name) ? ; let result = asn1 :: write_single (& name) ? ; Ok (pyo3 :: types :: PyBytes :: new (py , & result)) }
};
}
