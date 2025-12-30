// Generated macro for load_der_x509_crl (function)
macro_rules! Depcrate_x509_crlload_der_x509_crl {
() => {
// Module: crate::x509::crl
// Provides: {"load_der_x509_crl"}
// Dependencies: {}
# [pyo3 :: pyfunction] # [pyo3 (signature = (data , backend = None))] pub (crate) fn load_der_x509_crl (py : pyo3 :: Python < '_ > , data : pyo3 :: Py < pyo3 :: types :: PyBytes > , backend : Option < pyo3 :: Bound < '_ , pyo3 :: PyAny > > ,) -> Result < CertificateRevocationList , CryptographyError > { let _ = backend ; let owned = OwnedCertificateRevocationList :: try_new (data , | data | { asn1 :: parse_single (data . as_bytes (py)) }) ? ; let version = owned . borrow_dependent () . tbs_cert_list . version . unwrap_or (1) ; if version != 1 { return Err (CryptographyError :: from (exceptions :: InvalidVersion :: new_err ((format ! ("{version} is not a valid CRL version") , version ,)) ,)) ; } Ok (CertificateRevocationList { owned , revoked_certs : pyo3 :: sync :: PyOnceLock :: new () , cached_extensions : pyo3 :: sync :: PyOnceLock :: new () , }) }
};
}
