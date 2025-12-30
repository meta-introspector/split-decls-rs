// Generated macro for try_map_crl_to_revoked_cert (function)
macro_rules! Depcrate_x509_crltry_map_crl_to_revoked_cert {
() => {
// Module: crate::x509::crl
// Provides: {"try_map_crl_to_revoked_cert"}
// Dependencies: {}
fn try_map_crl_to_revoked_cert < F > (source : & OwnedCertificateRevocationList , py : pyo3 :: Python < '_ > , f : F ,) -> Option < OwnedRevokedCertificate > where F : for < 'a > FnOnce (& 'a RawCertificateRevocationList < 'a >) -> Option < RawRevokedCertificate < 'a > > , { OwnedRevokedCertificate :: try_new (source . borrow_owner () . clone_ref (py) , | _ | { match f (unsafe { std :: mem :: transmute :: < & RawCertificateRevocationList < '_ > , & RawCertificateRevocationList < '_ > , > (source . borrow_dependent ()) }) { Some (cert) => Ok (cert) , None => Err (()) , } }) . ok () }
};
}
