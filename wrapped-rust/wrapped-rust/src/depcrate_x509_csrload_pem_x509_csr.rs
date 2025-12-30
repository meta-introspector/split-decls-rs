// Generated macro for load_pem_x509_csr (function)
macro_rules! Depcrate_x509_csrload_pem_x509_csr {
() => {
// Module: crate::x509::csr
// Provides: {"load_pem_x509_csr"}
// Dependencies: {}
# [pyo3 :: pyfunction] # [pyo3 (signature = (data , backend = None))] pub (crate) fn load_pem_x509_csr (py : pyo3 :: Python < '_ > , data : & [u8] , backend : Option < pyo3 :: Bound < '_ , pyo3 :: PyAny > > ,) -> CryptographyResult < CertificateSigningRequest > { let _ = backend ; let parsed = x509 :: find_in_pem (data , | p | p . tag () == "CERTIFICATE REQUEST" || p . tag () == "NEW CERTIFICATE REQUEST" , "Valid PEM but no BEGIN CERTIFICATE REQUEST/END CERTIFICATE REQUEST delimiters. Are you sure this is a CSR?" ,) ? ; load_der_x509_csr (py , pyo3 :: types :: PyBytes :: new (py , parsed . contents ()) . unbind () , None ,) }
};
}
