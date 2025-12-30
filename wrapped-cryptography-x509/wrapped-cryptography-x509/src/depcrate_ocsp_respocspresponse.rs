// Generated macro for OCSPResponse (struct)
macro_rules! Depcrate_ocsp_respOCSPResponse {
() => {
// Module: crate::ocsp_resp
// Provides: {"OCSPResponse"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct OCSPResponse < 'a > { pub response_status : asn1 :: Enumerated , # [explicit (0)] pub response_bytes : Option < ResponseBytes < 'a > > , }
};
}
