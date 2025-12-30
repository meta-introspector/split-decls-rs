// Generated macro for OCSPRequest (struct)
macro_rules! Depcrate_ocsp_reqOCSPRequest {
() => {
// Module: crate::ocsp_req
// Provides: {"OCSPRequest"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct OCSPRequest < 'a > { pub tbs_request : TBSRequest < 'a > , # [explicit (0)] pub optional_signature : Option < asn1 :: Sequence < 'a > > , }
};
}
