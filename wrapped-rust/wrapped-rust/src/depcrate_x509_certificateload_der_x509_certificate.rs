// Generated macro for load_der_x509_certificate (function)
macro_rules! Depcrate_x509_certificateload_der_x509_certificate {
() => {
// Module: crate::x509::certificate
// Provides: {"load_der_x509_certificate"}
// Dependencies: {}
# [pyo3 :: pyfunction] # [pyo3 (signature = (data , backend = None))] pub (crate) fn load_der_x509_certificate (py : pyo3 :: Python < '_ > , data : pyo3 :: Py < pyo3 :: types :: PyBytes > , backend : Option < pyo3 :: Bound < '_ , pyo3 :: PyAny > > ,) -> CryptographyResult < Certificate > { let _ = backend ; let raw = OwnedCertificate :: try_new (data , | data | asn1 :: parse_single (data . as_bytes (py))) ? ; cert_version (py , raw . borrow_dependent () . tbs_cert . version) ? ; warn_if_not_positive (py , raw . borrow_dependent () . tbs_cert . serial . as_bytes ()) ? ; warn_if_invalid_params (py , raw . borrow_dependent () . signature_alg . params . clone ()) ? ; warn_if_invalid_params (py , raw . borrow_dependent () . tbs_cert . signature_alg . params . clone () ,) ? ; Ok (Certificate { raw , cached_extensions : pyo3 :: sync :: PyOnceLock :: new () , }) }
};
}
