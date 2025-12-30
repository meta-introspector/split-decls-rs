// Generated macro for oid_from_py_extension_type (function)
macro_rules! Depcrate_x509_verify_extension_policyoid_from_py_extension_type {
() => {
// Module: crate::x509::verify::extension_policy
// Provides: {"oid_from_py_extension_type"}
// Dependencies: {}
fn oid_from_py_extension_type (py : pyo3 :: Python < '_ > , extension_type : pyo3 :: Bound < '_ , pyo3 :: types :: PyType > ,) -> pyo3 :: PyResult < asn1 :: ObjectIdentifier > { if ! extension_type . is_subclass (& types :: EXTENSION_TYPE . get (py) ?) ? { return Err (pyo3 :: exceptions :: PyTypeError :: new_err ("extension_type must be a subclass of ExtensionType" ,)) ; } py_oid_to_oid (extension_type . getattr (intern ! (py , "oid")) ?) }
};
}
