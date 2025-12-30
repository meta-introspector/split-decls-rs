// Generated macro for map_revoked_cert (function)
macro_rules! Depcrate_x509_crlmap_revoked_cert {
() => {
// Module: crate::x509::crl
// Provides: {"map_revoked_cert"}
// Dependencies: {}
fn map_revoked_cert < F > (source : & OwnedRevokedCertificate , py : pyo3 :: Python < '_ > , f : F ,) -> OwnedRevokedCertificate where F : for < 'a > FnOnce (& 'a crl :: RevokedCertificate < 'a >) -> crl :: RevokedCertificate < 'a > , { OwnedRevokedCertificate :: new (source . borrow_owner () . clone_ref (py) , | _ | { f (unsafe { std :: mem :: transmute :: < & crl :: RevokedCertificate < '_ > , & crl :: RevokedCertificate < '_ > > (source . borrow_dependent () ,) }) }) }
};
}
