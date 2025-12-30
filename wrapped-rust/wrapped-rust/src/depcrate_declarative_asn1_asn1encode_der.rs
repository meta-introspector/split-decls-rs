// Generated macro for encode_der (function)
macro_rules! Depcrate_declarative_asn1_asn1encode_der {
() => {
// Module: crate::declarative_asn1::asn1
// Provides: {"encode_der"}
// Dependencies: {}
# [pyo3 :: pyfunction] pub (crate) fn encode_der < 'p > (py : pyo3 :: Python < 'p > , value : & pyo3 :: Bound < 'p , pyo3 :: types :: PyAny > ,) -> pyo3 :: PyResult < pyo3 :: Bound < 'p , pyo3 :: types :: PyBytes > > { let class = value . get_type () ; let annotated_type = asn1_types :: python_class_to_annotated (py , & class) ? ; let object = asn1_types :: AnnotatedTypeObject { annotated_type : annotated_type . get () , value : value . clone () , } ; let encoded_bytes = asn1 :: write (| writer | object . write (writer)) . map_err (| e | pyo3 :: exceptions :: PyValueError :: new_err (e . to_string ())) ? ; Ok (pyo3 :: types :: PyBytes :: new (py , & encoded_bytes)) }
};
}
