// Generated macro for make_py_extension (function)
macro_rules! Depcrate_x509_verify_extension_policymake_py_extension {
() => {
// Module: crate::x509::verify::extension_policy
// Provides: {"make_py_extension"}
// Dependencies: {}
fn make_py_extension < 'chain , 'p > (py : pyo3 :: Python < 'p > , ext : Option < & Extension < 'p > > ,) -> ValidationResult < 'chain , Option < pyo3 :: Bound < 'p , pyo3 :: types :: PyAny > > , PyCryptoOps > { Ok (match ext { None => None , Some (ext) => parse_cert_ext (py , ext) . map_err (| e | { ValidationError :: new (ValidationErrorKind :: Other (format ! ("{e} (while converting Extension to Python object)"))) }) ? , }) }
};
}
