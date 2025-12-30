// Generated macro for encode_extension_value (function)
macro_rules! Depcrate_x509_commonencode_extension_value {
() => {
// Module: crate::x509::common
// Provides: {"encode_extension_value"}
// Dependencies: {}
# [pyo3 :: pyfunction] pub (crate) fn encode_extension_value < 'p > (py : pyo3 :: Python < 'p > , py_ext : pyo3 :: Bound < 'p , pyo3 :: PyAny > ,) -> pyo3 :: PyResult < pyo3 :: Bound < 'p , pyo3 :: types :: PyBytes > > { let oid = py_oid_to_oid (py_ext . getattr (pyo3 :: intern ! (py , "oid")) ?) ? ; if let Some (data) = x509 :: extensions :: encode_extension (py , & oid , & py_ext) ? { let py_data = pyo3 :: types :: PyBytes :: new (py , & data) ; return Ok (py_data) ; } Err (pyo3 :: exceptions :: PyNotImplementedError :: new_err (format ! ("Extension not supported: {oid}"))) }
};
}
