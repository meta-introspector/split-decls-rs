// Generated macro for encode_naming_authority (function)
macro_rules! Depcrate_x509_extensionsencode_naming_authority {
() => {
// Module: crate::x509::extensions
// Provides: {"encode_naming_authority"}
// Dependencies: {}
fn encode_naming_authority < 'a > (py : pyo3 :: Python < '_ > , ka_str : & 'a cryptography_keepalive :: KeepAlive < pyo3 :: pybacked :: PyBackedStr > , py_naming_authority : & pyo3 :: Bound < 'a , pyo3 :: PyAny > ,) -> CryptographyResult < extensions :: NamingAuthority < 'a > > { let py_oid = py_naming_authority . getattr (pyo3 :: intern ! (py , "id")) ? ; let id = if ! py_oid . is_none () { Some (py_oid_to_oid (py_oid) ?) } else { None } ; let py_url = py_naming_authority . getattr (pyo3 :: intern ! (py , "url")) ? ; let url = if ! py_url . is_none () { let py_url_str = ka_str . add (py_url . extract :: < PyBackedStr > () ?) ; match asn1 :: IA5String :: new (py_url_str) { Some (s) => Some (s) , None => { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("url value must be a valid IA5String") ,)) } } } else { None } ; let py_text = py_naming_authority . getattr (pyo3 :: intern ! (py , "text")) ? ; let text = if ! py_text . is_none () { let py_text_str = ka_str . add (py_text . extract :: < PyBackedStr > () ?) ; Some (extensions :: DisplayText :: Utf8String (asn1 :: Utf8String :: new (py_text_str ,))) } else { None } ; Ok (extensions :: NamingAuthority { id , url , text }) }
};
}
