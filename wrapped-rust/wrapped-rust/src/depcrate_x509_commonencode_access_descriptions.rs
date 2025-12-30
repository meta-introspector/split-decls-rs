// Generated macro for encode_access_descriptions (function)
macro_rules! Depcrate_x509_commonencode_access_descriptions {
() => {
// Module: crate::x509::common
// Provides: {"encode_access_descriptions"}
// Dependencies: {}
pub (crate) fn encode_access_descriptions < 'a > (py : pyo3 :: Python < 'a > , py_ads : & pyo3 :: Bound < 'a , pyo3 :: PyAny > ,) -> CryptographyResult < Vec < u8 > > { let mut ads = vec ! [] ; let ka_bytes = cryptography_keepalive :: KeepAlive :: new () ; let ka_str = cryptography_keepalive :: KeepAlive :: new () ; for py_ad in py_ads . try_iter () ? { let py_ad = py_ad ? ; let py_oid = py_ad . getattr (pyo3 :: intern ! (py , "access_method")) ? ; let access_method = py_oid_to_oid (py_oid) ? ; let py_access_location = py_ad . getattr (pyo3 :: intern ! (py , "access_location")) ? ; let access_location = encode_general_name (py , & ka_bytes , & ka_str , & py_access_location) ? ; ads . push (AccessDescription { access_method , access_location , }) ; } Ok (asn1 :: write_single (& asn1 :: SequenceOfWriter :: new (ads)) ?) }
};
}
