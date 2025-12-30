// Generated macro for map_arc_data_ocsp_response (function)
macro_rules! Depcrate_x509_ocsp_respmap_arc_data_ocsp_response {
() => {
// Module: crate::x509::ocsp_resp
// Provides: {"map_arc_data_ocsp_response"}
// Dependencies: {}
fn map_arc_data_ocsp_response (py : pyo3 :: Python < '_ > , it : & OwnedOCSPResponse , f : impl for < 'this > FnOnce (& 'this [u8] , & ocsp_resp :: OCSPResponse < 'this > ,) -> cryptography_x509 :: certificate :: Certificate < 'this > ,) -> certificate :: OwnedCertificate { certificate :: OwnedCertificate :: new (it . borrow_owner () . clone_ref (py) , | inner_it | { it . with_dependent (| _ , value | { f (inner_it . as_bytes (py) , unsafe { std :: mem :: transmute :: < & ocsp_resp :: OCSPResponse < '_ > , & ocsp_resp :: OCSPResponse < '_ > > (value ,) }) }) }) }
};
}
