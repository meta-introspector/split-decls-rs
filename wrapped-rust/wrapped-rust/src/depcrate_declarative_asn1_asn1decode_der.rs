// Generated macro for decode_der (function)
macro_rules! Depcrate_declarative_asn1_asn1decode_der {
() => {
// Module: crate::declarative_asn1::asn1
// Provides: {"decode_der"}
// Dependencies: {}
# [pyo3 :: pyfunction] pub (crate) fn decode_der < 'p > (py : pyo3 :: Python < 'p > , class : & pyo3 :: Bound < 'p , pyo3 :: types :: PyType > , value : & 'p [u8] ,) -> pyo3 :: PyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { asn1 :: parse (value , | parser | { let annotated_type = asn1_types :: python_class_to_annotated (py , class) ? ; decode_annotated_type (py , parser , annotated_type . get ()) }) . map_err (| e | pyo3 :: exceptions :: PyValueError :: new_err (e . to_string ())) }
};
}
