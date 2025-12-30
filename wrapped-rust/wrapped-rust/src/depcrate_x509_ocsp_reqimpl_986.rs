// Generated macro for impl_986 (impl)
macro_rules! Depcrate_x509_ocsp_reqimpl_986 {
() => {
// Module: crate::x509::ocsp_req
// Provides: {"impl_986"}
// Dependencies: {}
impl OCSPRequest { fn cert_id (& self) -> ocsp_req :: CertID < '_ > { self . raw . borrow_dependent () . tbs_request . request_list . unwrap_read () . clone () . next () . unwrap () . req_cert } }
};
}
