// Generated macro for load_der_ocsp_response (function)
macro_rules! Depcrate_x509_ocsp_respload_der_ocsp_response {
() => {
// Module: crate::x509::ocsp_resp
// Provides: {"load_der_ocsp_response"}
// Dependencies: {}
# [pyo3 :: pyfunction] pub (crate) fn load_der_ocsp_response (py : pyo3 :: Python < '_ > , data : pyo3 :: Py < pyo3 :: types :: PyBytes > ,) -> Result < OCSPResponse , CryptographyError > { let raw = OwnedOCSPResponse :: try_new (data , | data | asn1 :: parse_single (data . as_bytes (py))) ? ; let response = raw . borrow_dependent () ; match response . response_status . value () { SUCCESSFUL_RESPONSE => match response . response_bytes { Some (ref bytes) => { if bytes . response_type != BASIC_RESPONSE_OID { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("Successful OCSP response does not contain a BasicResponse" ,) ,)) ; } } None => { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("Successful OCSP response does not contain a BasicResponse" ,) ,)) } } , MALFORMED_REQUEST_RESPONSE | INTERNAL_ERROR_RESPONSE | TRY_LATER_RESPONSE | SIG_REQUIRED_RESPONSE | UNAUTHORIZED_RESPONSE => { } _ => { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("OCSP response has an unknown status code") ,)) } } ; Ok (OCSPResponse { raw : Arc :: new (raw) , cached_extensions : pyo3 :: sync :: PyOnceLock :: new () , cached_single_extensions : pyo3 :: sync :: PyOnceLock :: new () , }) }
};
}
