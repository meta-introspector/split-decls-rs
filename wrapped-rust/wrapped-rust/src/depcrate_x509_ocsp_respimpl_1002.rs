// Generated macro for impl_1002 (impl)
macro_rules! Depcrate_x509_ocsp_respimpl_1002 {
() => {
// Module: crate::x509::ocsp_resp
// Provides: {"impl_1002"}
// Dependencies: {}
impl OCSPResponse { fn requires_successful_response (& self) -> pyo3 :: PyResult < & ocsp_resp :: BasicOCSPResponse < '_ > > { match self . raw . borrow_dependent () . response_bytes . as_ref () { Some (b) => Ok (b . response . get ()) , None => Err (pyo3 :: exceptions :: PyValueError :: new_err ("OCSP response status is not successful so the property has no value" ,)) , } } }
};
}
