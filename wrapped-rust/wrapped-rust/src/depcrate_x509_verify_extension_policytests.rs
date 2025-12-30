// Generated macro for tests (module)
macro_rules! Depcrate_x509_verify_extension_policytests {
() => {
// Module: crate::x509::verify::extension_policy
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use cryptography_x509 :: extensions :: Extension ; # [test] fn test_make_py_extension_fail () { pyo3 :: Python :: attach (| py | { let invalid_extension = Extension { extn_id : asn1 :: ObjectIdentifier :: from_string ("2.5.29.17") . unwrap () , critical : false , extn_value : & [] , } ; let result = super :: make_py_extension (py , Some (& invalid_extension)) ; assert ! (result . is_err ()) ; let error = result . unwrap_err () ; assert ! (format ! ("{error}") . contains ("(while converting Extension to Python object)")) ; }) } }
};
}
