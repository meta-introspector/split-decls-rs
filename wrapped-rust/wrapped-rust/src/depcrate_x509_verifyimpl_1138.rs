// Generated macro for impl_1138 (impl)
macro_rules! Depcrate_x509_verifyimpl_1138 {
() => {
// Module: crate::x509::verify
// Provides: {"impl_1138"}
// Dependencies: {}
# [pyo3 :: pymethods] impl PyClientVerifier { fn verify (& self , py : pyo3 :: Python < '_ > , leaf : pyo3 :: Py < PyCertificate > , intermediates : Vec < pyo3 :: Py < PyCertificate > > ,) -> CryptographyResult < PyVerifiedClient > { let policy = Policy :: new (self . as_policy_def () , self . py_policy . clone_ref (py)) ; let store = self . store . get () ; let intermediates = intermediates . iter () . map (| i | VerificationCertificate :: new (i . get () . raw . borrow_dependent () , i . clone_ref (py))) . collect :: < Vec < _ > > () ; let v = VerificationCertificate :: new (leaf . get () . raw . borrow_dependent () , leaf . clone_ref (py)) ; let chain = cryptography_x509_verification :: verify (& v , & intermediates , & policy , store . raw . borrow_dependent () ,) . or_else (| e | handle_validation_error (py , e)) ? ; let py_chain = pyo3 :: types :: PyList :: empty (py) ; for c in & chain { py_chain . append (c . extra ()) ? ; } let subjects = match & chain [0] . certificate () . extensions () . ok () . unwrap () . get_extension (& SUBJECT_ALTERNATIVE_NAME_OID) { Some (leaf_san) => { let leaf_gns = leaf_san . value :: < SubjectAlternativeName < '_ > > () ? ; Some (parse_general_names (py , & leaf_gns) ? . unbind ()) } None => None , } ; Ok (PyVerifiedClient { subjects , chain : py_chain . unbind () , }) } }
};
}
