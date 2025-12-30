// Generated macro for impl_1027 (impl)
macro_rules! Depcrate_x509_ocsp_respimpl_1027 {
() => {
// Module: crate::x509::ocsp_resp
// Provides: {"impl_1027"}
// Dependencies: {}
# [pyo3 :: pymethods] impl OCSPResponseIterator { fn __iter__ (slf : pyo3 :: PyRef < '_ , Self >) -> pyo3 :: PyRef < '_ , Self > { slf } fn __next__ (& mut self) -> Option < OCSPSingleResponse > { let single_resp = try_map_arc_data_mut_ocsp_response_iterator (& mut self . contents , | _data , v | { match v . next () { Some (single_resp) => Ok (single_resp) , None => Err (()) , } }) . ok () ? ; Some (OCSPSingleResponse { raw : single_resp }) } }
};
}
