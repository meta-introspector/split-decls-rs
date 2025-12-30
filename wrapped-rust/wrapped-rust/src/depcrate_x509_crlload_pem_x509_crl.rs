// Generated macro for load_pem_x509_crl (function)
macro_rules! Depcrate_x509_crlload_pem_x509_crl {
() => {
// Module: crate::x509::crl
// Provides: {"load_pem_x509_crl"}
// Dependencies: {}
# [pyo3 :: pyfunction] # [pyo3 (signature = (data , backend = None))] pub (crate) fn load_pem_x509_crl (py : pyo3 :: Python < '_ > , data : & [u8] , backend : Option < pyo3 :: Bound < '_ , pyo3 :: PyAny > > ,) -> Result < CertificateRevocationList , CryptographyError > { let _ = backend ; let block = x509 :: find_in_pem (data , | p | p . tag () == "X509 CRL" , "Valid PEM but no BEGIN X509 CRL/END X509 delimiters. Are you sure this is a CRL?" ,) ? ; load_der_x509_crl (py , pyo3 :: types :: PyBytes :: new (py , block . contents ()) . unbind () , None ,) }
};
}
