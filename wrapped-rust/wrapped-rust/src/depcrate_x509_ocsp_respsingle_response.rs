// Generated macro for single_response (function)
macro_rules! Depcrate_x509_ocsp_respsingle_response {
() => {
// Module: crate::x509::ocsp_resp
// Provides: {"single_response"}
// Dependencies: {}
fn single_response < 'a > (resp : & ocsp_resp :: BasicOCSPResponse < 'a > ,) -> Result < ocsp_resp :: SingleResponse < 'a > , CryptographyError > { let responses = resp . tbs_response_data . responses . unwrap_read () ; let num_responses = responses . len () ; if num_responses != 1 { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err (format ! ("OCSP response contains {num_responses} SINGLERESP structures.  Use .response_iter to iterate through them")))) ; } Ok (responses . clone () . next () . unwrap ()) }
};
}
