// Generated macro for build_subject_owner (function)
macro_rules! Depcrate_x509_verifybuild_subject_owner {
() => {
// Module: crate::x509::verify
// Provides: {"build_subject_owner"}
// Dependencies: {}
fn build_subject_owner (py : pyo3 :: Python < '_ > , subject : & pyo3 :: Py < pyo3 :: PyAny > ,) -> pyo3 :: PyResult < SubjectOwner > { let subject = subject . bind (py) ; if subject . is_instance (& types :: DNS_NAME . get (py) ?) ? { let value = subject . getattr (pyo3 :: intern ! (py , "value")) ? . extract :: < String > () ? ; Ok (SubjectOwner :: DNSName (value)) } else if subject . is_instance (& types :: IP_ADDRESS . get (py) ?) ? { let value = subject . getattr (pyo3 :: intern ! (py , "_packed")) ? . call0 () ? . cast :: < pyo3 :: types :: PyBytes > () ? . clone () ; Ok (SubjectOwner :: IPAddress (value . unbind ())) } else { Err (pyo3 :: exceptions :: PyTypeError :: new_err ("unsupported subject type" ,)) } }
};
}
