// Generated macro for encode_authority_key_identifier (function)
macro_rules! Depcrate_x509_extensionsencode_authority_key_identifier {
() => {
// Module: crate::x509::extensions
// Provides: {"encode_authority_key_identifier"}
// Dependencies: {}
pub (crate) fn encode_authority_key_identifier < 'a > (py : pyo3 :: Python < 'a > , py_aki : & pyo3 :: Bound < 'a , pyo3 :: PyAny > ,) -> CryptographyResult < Vec < u8 > > { # [derive (pyo3 :: FromPyObject)] struct PyAuthorityKeyIdentifier < 'a > { key_identifier : Option < pyo3 :: pybacked :: PyBackedBytes > , authority_cert_issuer : Option < pyo3 :: Bound < 'a , pyo3 :: PyAny > > , authority_cert_serial_number : Option < pyo3 :: Bound < 'a , pyo3 :: types :: PyInt > > , } let aki = py_aki . extract :: < PyAuthorityKeyIdentifier < '_ > > () ? ; let ka_bytes = cryptography_keepalive :: KeepAlive :: new () ; let ka_str = cryptography_keepalive :: KeepAlive :: new () ; let authority_cert_issuer = if let Some (authority_cert_issuer) = aki . authority_cert_issuer { let gns = x509 :: common :: encode_general_names (py , & ka_bytes , & ka_str , & authority_cert_issuer) ? ; Some (asn1 :: SequenceOfWriter :: new (gns)) } else { None } ; let serial_bytes ; let authority_cert_serial_number = if let Some (authority_cert_serial_number) = aki . authority_cert_serial_number { serial_bytes = py_uint_to_big_endian_bytes (py , authority_cert_serial_number) ? ; Some (SerialNumber :: new (& serial_bytes) . unwrap ()) } else { None } ; Ok (asn1 :: write_single (& extensions :: AuthorityKeyIdentifier :: < Asn1Write , > { authority_cert_issuer , authority_cert_serial_number , key_identifier : aki . key_identifier . as_deref () , }) ?) }
};
}
