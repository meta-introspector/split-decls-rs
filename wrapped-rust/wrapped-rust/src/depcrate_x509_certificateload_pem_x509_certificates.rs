// Generated macro for load_pem_x509_certificates (function)
macro_rules! Depcrate_x509_certificateload_pem_x509_certificates {
() => {
// Module: crate::x509::certificate
// Provides: {"load_pem_x509_certificates"}
// Dependencies: {}
# [pyo3 :: pyfunction] pub (crate) fn load_pem_x509_certificates (py : pyo3 :: Python < '_ > , data : & [u8] ,) -> CryptographyResult < Vec < Certificate > > { let certs = pem :: parse_many (data) ? . iter () . filter (| p | p . tag () == "CERTIFICATE" || p . tag () == "X509 CERTIFICATE") . map (| p | { load_der_x509_certificate (py , pyo3 :: types :: PyBytes :: new (py , p . contents ()) . unbind () , None ,) }) . collect :: < Result < Vec < _ > , _ > > () ? ; if certs . is_empty () { return Err (CryptographyError :: from (pem :: PemError :: MalformedFraming)) ; } Ok (certs) }
};
}
