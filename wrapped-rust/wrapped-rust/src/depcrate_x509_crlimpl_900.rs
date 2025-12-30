// Generated macro for impl_900 (impl)
macro_rules! Depcrate_x509_crlimpl_900 {
() => {
// Module: crate::x509::crl
// Provides: {"impl_900"}
// Dependencies: {}
impl CertificateRevocationList { fn public_bytes_der (& self) -> CryptographyResult < Vec < u8 > > { Ok (asn1 :: write_single (self . owned . borrow_dependent ()) ?) } fn revoked_cert (& self , py : pyo3 :: Python < '_ > , idx : usize) -> RevokedCertificate { RevokedCertificate { owned : self . revoked_certs . get (py) . unwrap () [idx] . clone_with_py (py) , cached_extensions : pyo3 :: sync :: PyOnceLock :: new () , } } fn len (& self) -> usize { self . owned . borrow_dependent () . tbs_cert_list . revoked_certificates . as_ref () . map_or (0 , | v | v . unwrap_read () . len ()) } }
};
}
