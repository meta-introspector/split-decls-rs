// Generated macro for try_map_arc_data_mut_crl_iterator (function)
macro_rules! Depcrate_x509_crltry_map_arc_data_mut_crl_iterator {
() => {
// Module: crate::x509::crl
// Provides: {"try_map_arc_data_mut_crl_iterator"}
// Dependencies: {}
fn try_map_arc_data_mut_crl_iterator < E > (py : pyo3 :: Python < '_ > , it : & mut OwnedCRLIteratorData , f : impl for < 'this > FnOnce (& mut Option < asn1 :: SequenceOf < 'this , crl :: RevokedCertificate < 'this > > > ,) -> Result < crl :: RevokedCertificate < 'this > , E > ,) -> Result < OwnedRevokedCertificate , E > { OwnedRevokedCertificate :: try_new (it . borrow_owner () . clone_ref (py) , | _pybytes | { it . with_dependent_mut (| _ , value | { f (unsafe { std :: mem :: transmute :: < & mut Option < asn1 :: SequenceOf < '_ , crl :: RevokedCertificate < '_ > > > , & mut Option < asn1 :: SequenceOf < '_ , crl :: RevokedCertificate < '_ > > > , > (value) }) }) }) }
};
}
