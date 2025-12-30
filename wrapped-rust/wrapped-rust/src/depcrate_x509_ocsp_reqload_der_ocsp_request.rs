// Generated macro for load_der_ocsp_request (function)
macro_rules! Depcrate_x509_ocsp_reqload_der_ocsp_request {
() => {
// Module: crate::x509::ocsp_req
// Provides: {"load_der_ocsp_request"}
// Dependencies: {}
# [pyo3 :: pyfunction] pub (crate) fn load_der_ocsp_request (py : pyo3 :: Python < '_ > , data : pyo3 :: Py < pyo3 :: types :: PyBytes > ,) -> CryptographyResult < OCSPRequest > { let raw = OwnedOCSPRequest :: try_new (data , | data | asn1 :: parse_single (data . as_bytes (py))) ? ; if raw . borrow_dependent () . tbs_request . request_list . unwrap_read () . len () != 1 { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyNotImplementedError :: new_err ("OCSP request contains more than one request" ,) ,)) ; } Ok (OCSPRequest { raw , cached_extensions : pyo3 :: sync :: PyOnceLock :: new () , }) }
};
}
