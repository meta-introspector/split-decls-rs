// Generated macro for map_crl_to_iterator_data (function)
macro_rules! Depcrate_x509_crlmap_crl_to_iterator_data {
() => {
// Module: crate::x509::crl
// Provides: {"map_crl_to_iterator_data"}
// Dependencies: {}
fn map_crl_to_iterator_data < F > (source : & OwnedCertificateRevocationList , py : pyo3 :: Python < '_ > , f : F ,) -> OwnedCRLIteratorData where F : for < 'a > FnOnce (& 'a RawCertificateRevocationList < 'a > ,) -> Option < asn1 :: SequenceOf < 'a , crl :: RevokedCertificate < 'a > > > , { OwnedCRLIteratorData :: new (source . borrow_owner () . clone_ref (py) , | _ | { f (unsafe { std :: mem :: transmute :: < & RawCertificateRevocationList < '_ > , & RawCertificateRevocationList < '_ > , > (source . borrow_dependent ()) }) }) }
};
}
