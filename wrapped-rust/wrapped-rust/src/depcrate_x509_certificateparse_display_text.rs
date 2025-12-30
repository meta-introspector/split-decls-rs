// Generated macro for parse_display_text (function)
macro_rules! Depcrate_x509_certificateparse_display_text {
() => {
// Module: crate::x509::certificate
// Provides: {"parse_display_text"}
// Dependencies: {}
fn parse_display_text < 'p > (py : pyo3 :: Python < 'p > , text : DisplayText < '_ > ,) -> pyo3 :: PyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { match text { DisplayText :: IA5String (o) => Ok (pyo3 :: types :: PyString :: new (py , o . as_str ()) . into_any ()) , DisplayText :: Utf8String (o) => Ok (pyo3 :: types :: PyString :: new (py , o . as_str ()) . into_any ()) , DisplayText :: VisibleString (o) => { if asn1 :: VisibleString :: new (o . as_str ()) . is_none () { let warning_cls = types :: DEPRECATED_IN_41 . get (py) ? ; let message = c"Invalid ASN.1 (UTF-8 characters in a VisibleString) in the explicit text and/or notice reference of the certificate policies extension. In a future version of cryptography, an exception will be raised." ; pyo3 :: PyErr :: warn (py , & warning_cls , message , 1) ? ; } Ok (pyo3 :: types :: PyString :: new (py , o . as_str ()) . into_any ()) } DisplayText :: BmpString (o) => { let py_bytes = pyo3 :: types :: PyBytes :: new (py , o . as_utf16_be_bytes ()) ; Ok (py_bytes . call_method1 (pyo3 :: intern ! (py , "decode") , (pyo3 :: intern ! (py , "utf_16_be") ,) ,) ?) } } }
};
}
