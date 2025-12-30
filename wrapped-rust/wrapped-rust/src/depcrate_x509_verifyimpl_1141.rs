// Generated macro for impl_1141 (impl)
macro_rules! Depcrate_x509_verifyimpl_1141 {
() => {
// Module: crate::x509::verify
// Provides: {"impl_1141"}
// Dependencies: {}
# [pyo3 :: pymethods] impl PyServerVerifier { fn verify < 'p > (& self , py : pyo3 :: Python < 'p > , leaf : pyo3 :: Py < PyCertificate > , intermediates : Vec < pyo3 :: Py < PyCertificate > > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: types :: PyList > > { let policy = Policy :: new (self . as_policy_def () , self . py_policy . clone_ref (py)) ; let store = self . store . get () ; let intermediates = intermediates . iter () . map (| i | VerificationCertificate :: new (i . get () . raw . borrow_dependent () , i . clone_ref (py))) . collect :: < Vec < _ > > () ; let v = VerificationCertificate :: new (leaf . get () . raw . borrow_dependent () , leaf . clone_ref (py)) ; let chain = cryptography_x509_verification :: verify (& v , & intermediates , & policy , store . raw . borrow_dependent () ,) . or_else (| e | handle_validation_error (py , e)) ? ; let result = pyo3 :: types :: PyList :: empty (py) ; for c in chain { result . append (c . extra ()) ? ; } Ok (result) } }
};
}
