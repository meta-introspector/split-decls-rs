// Generated macro for invoke_py_validator_callback (function)
macro_rules! Depcrate_x509_verify_extension_policyinvoke_py_validator_callback {
() => {
// Module: crate::x509::verify::extension_policy
// Provides: {"invoke_py_validator_callback"}
// Dependencies: {}
fn invoke_py_validator_callback < 'py > (py : pyo3 :: Python < 'py > , py_cb : & pyo3 :: Py < pyo3 :: PyAny > , args : impl pyo3 :: call :: PyCallArgs < 'py > ,) -> ValidationResult < 'static , () , PyCryptoOps > { let result = py_cb . bind (py) . call1 (args) . map_err (| e | { ValidationError :: new (ValidationErrorKind :: Other (format ! ("Python extension validator failed: {e}" ,))) }) ? ; if ! result . is_none () { let error_kind = ValidationErrorKind :: Other ("Python validator must return None." . to_string ()) ; Err (ValidationError :: new (error_kind)) } else { Ok (()) } }
};
}
