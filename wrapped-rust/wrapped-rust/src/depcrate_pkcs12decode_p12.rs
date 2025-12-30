// Generated macro for decode_p12 (function)
macro_rules! Depcrate_pkcs12decode_p12 {
() => {
// Module: crate::pkcs12
// Provides: {"decode_p12"}
// Dependencies: {}
fn decode_p12 (py : pyo3 :: Python < '_ > , data : CffiBuf < '_ > , password : Option < CffiBuf < '_ > > ,) -> CryptographyResult < openssl :: pkcs12 :: ParsedPkcs12_2 > { let p12 = openssl :: pkcs12 :: Pkcs12 :: from_der (data . as_bytes ()) . map_err (| _ | { pyo3 :: exceptions :: PyValueError :: new_err ("Could not deserialize PKCS12 data") }) ? ; let password = if let Some (p) = password . as_ref () { std :: str :: from_utf8 (p . as_bytes ()) . map_err (| _ | pyo3 :: exceptions :: PyUnicodeDecodeError :: new_err (())) ? } else { "" } ; let parsed = p12 . parse2 (password) . map_err (| _ | pyo3 :: exceptions :: PyValueError :: new_err ("Invalid password or PKCS12 data")) ? ; if let Err (e) = asn1 :: parse_single :: < cryptography_x509 :: pkcs12 :: Pfx < '_ > > (data . as_bytes ()) { let warning_cls = pyo3 :: exceptions :: PyUserWarning :: type_object (py) ; let message = std :: ffi :: CString :: new (format ! ("PKCS#12 bundle could not be parsed as DER, falling back to parsing as BER. Please file an issue at https://github.com/pyca/cryptography/issues explaining how your PKCS#12 bundle was created. In the future, this may become an exception. Error details: {e}")) . unwrap () ; pyo3 :: PyErr :: warn (py , & warning_cls , & message , 1) ? ; } Ok (parsed) }
};
}
