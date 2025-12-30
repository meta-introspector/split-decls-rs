// Generated macro for tests (module)
macro_rules! Depcrate_ocsptests {
() => {
// Module: crate::ocsp
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { get_sentinel_max_time , OcspCertId , OcspCertStatus , OcspResponse , OcspResponseStatus , } ; use crate :: hash :: MessageDigest ; use crate :: x509 :: X509 ; const OCSP_RESPONSE_NO_NEXTUPDATE : & [u8] = include_bytes ! ("../test/ocsp_resp_no_nextupdate.der") ; const OCSP_CA_CERT : & [u8] = include_bytes ! ("../test/ocsp_ca_cert.der") ; const OCSP_SUBJECT_CERT : & [u8] = include_bytes ! ("../test/ocsp_subject_cert.der") ; # [test] fn test_ocsp_no_next_update () { let response = OcspResponse :: from_der (OCSP_RESPONSE_NO_NEXTUPDATE) . unwrap () ; assert_eq ! (response . status () , OcspResponseStatus :: SUCCESSFUL) ; let ca_cert = X509 :: from_der (OCSP_CA_CERT) . unwrap () ; let subject_cert = X509 :: from_der (OCSP_SUBJECT_CERT) . unwrap () ; let basic = response . basic () . unwrap () ; let cert_id = OcspCertId :: from_cert (MessageDigest :: sha256 () , & subject_cert , & ca_cert) . unwrap () ; let status = basic . find_status (& cert_id) . expect ("find_status should find the status") ; assert ! (status . next_update () . is_none ()) ; # [allow (deprecated)] let deprecated_next = status . next_update ; let sentinel = get_sentinel_max_time () ; assert_eq ! (format ! ("{}" , deprecated_next) , format ! ("{}" , sentinel)) ; assert_eq ! (status . status , OcspCertStatus :: GOOD) ; } }
};
}
