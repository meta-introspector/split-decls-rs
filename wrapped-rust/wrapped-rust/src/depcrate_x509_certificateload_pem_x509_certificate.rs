// Generated macro for load_pem_x509_certificate (function)
macro_rules! Depcrate_x509_certificateload_pem_x509_certificate {
() => {
// Module: crate::x509::certificate
// Provides: {"load_pem_x509_certificate"}
// Dependencies: {}
# [pyo3 :: pyfunction] # [pyo3 (signature = (data , backend = None))] pub (crate) fn load_pem_x509_certificate (py : pyo3 :: Python < '_ > , data : & [u8] , backend : Option < pyo3 :: Bound < '_ , pyo3 :: PyAny > > ,) -> CryptographyResult < Certificate > { let _ = backend ; let parsed = x509 :: find_in_pem (data , | p | p . tag () == "CERTIFICATE" || p . tag () == "X509 CERTIFICATE" , "Valid PEM but no BEGIN CERTIFICATE/END CERTIFICATE delimiters. Are you sure this is a certificate?" ,) ? ; load_der_x509_certificate (py , pyo3 :: types :: PyBytes :: new (py , parsed . contents ()) . unbind () , None ,) }
};
}
