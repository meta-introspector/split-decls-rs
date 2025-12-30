// Generated macro for impl_1148 (impl)
macro_rules! Depcrate_x509_verifyimpl_1148 {
() => {
// Module: crate::x509::verify
// Provides: {"impl_1148"}
// Dependencies: {}
# [pyo3 :: pymethods] impl PyStore { # [new] fn new (py : pyo3 :: Python < '_ > , certs : Vec < pyo3 :: Py < PyCertificate > >) -> pyo3 :: PyResult < Self > { if certs . is_empty () { return Err (pyo3 :: exceptions :: PyValueError :: new_err ("can't create an empty store" ,)) ; } Ok (Self { raw : RawPyStore :: new (certs , | v | { Store :: new (v . iter () . map (| t | { VerificationCertificate :: new (t . get () . raw . borrow_dependent () , t . clone_ref (py)) })) }) , }) } }
};
}
