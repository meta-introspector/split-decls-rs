// Generated macro for try_map_arc_data_mut_ocsp_response_iterator (function)
macro_rules! Depcrate_x509_ocsp_resptry_map_arc_data_mut_ocsp_response_iterator {
() => {
// Module: crate::x509::ocsp_resp
// Provides: {"try_map_arc_data_mut_ocsp_response_iterator"}
// Dependencies: {}
fn try_map_arc_data_mut_ocsp_response_iterator < E > (it : & mut OwnedOCSPResponseIteratorData , f : impl for < 'this > FnOnce (& 'this OwnedOCSPResponse , & mut asn1 :: SequenceOf < 'this , ocsp_resp :: SingleResponse < 'this > > ,) -> Result < ocsp_resp :: SingleResponse < 'this > , E > ,) -> Result < OwnedSingleResponse , E > { OwnedSingleResponse :: try_new (Arc :: clone (it . borrow_owner ()) , | inner_it | { it . with_dependent_mut (| _ , value | { f (inner_it , unsafe { std :: mem :: transmute :: < & mut asn1 :: SequenceOf < '_ , ocsp_resp :: SingleResponse < '_ > > , & mut asn1 :: SequenceOf < '_ , ocsp_resp :: SingleResponse < '_ > > , > (value) }) }) }) }
};
}
