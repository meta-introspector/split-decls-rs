// Generated macro for load_der_x509_csr (function)
macro_rules! Depcrate_x509_csrload_der_x509_csr {
() => {
// Module: crate::x509::csr
// Provides: {"load_der_x509_csr"}
// Dependencies: {}
# [pyo3 :: pyfunction] # [pyo3 (signature = (data , backend = None))] pub (crate) fn load_der_x509_csr (py : pyo3 :: Python < '_ > , data : pyo3 :: Py < pyo3 :: types :: PyBytes > , backend : Option < pyo3 :: Bound < '_ , pyo3 :: PyAny > > ,) -> CryptographyResult < CertificateSigningRequest > { let _ = backend ; let raw = OwnedCsr :: try_new (data , | data | asn1 :: parse_single (data . as_bytes (py))) ? ; let version = raw . borrow_dependent () . csr_info . version ; if version != 0 { return Err (CryptographyError :: from (exceptions :: InvalidVersion :: new_err ((format ! ("{version} is not a valid CSR version") , version ,)) ,)) ; } Ok (CertificateSigningRequest { raw , cached_extensions : pyo3 :: sync :: PyOnceLock :: new () , }) }
};
}
