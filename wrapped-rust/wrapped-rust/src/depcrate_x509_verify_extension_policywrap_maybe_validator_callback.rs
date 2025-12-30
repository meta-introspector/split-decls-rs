// Generated macro for wrap_maybe_validator_callback (function)
macro_rules! Depcrate_x509_verify_extension_policywrap_maybe_validator_callback {
() => {
// Module: crate::x509::verify::extension_policy
// Provides: {"wrap_maybe_validator_callback"}
// Dependencies: {}
fn wrap_maybe_validator_callback (py_cb : pyo3 :: Py < pyo3 :: PyAny > ,) -> MaybeExtensionValidatorCallback < 'static , PyCryptoOps > { Arc :: new (move | policy : & Policy < '_ , PyCryptoOps > , cert : & VerificationCertificate < '_ , PyCryptoOps > , ext : Option < & Extension < '_ > > | { pyo3 :: Python :: attach (| py | { invoke_py_validator_callback (py , & py_cb , (policy . extra . clone_ref (py) , cert . extra () . clone_ref (py) , make_py_extension (py , ext) ? ,) ,) }) } ,) }
};
}
