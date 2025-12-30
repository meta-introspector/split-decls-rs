// Generated macro for SingleResponse (struct)
macro_rules! Depcrate_ocsp_respSingleResponse {
() => {
// Module: crate::ocsp_resp
// Provides: {"SingleResponse"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct SingleResponse < 'a > { pub cert_id : ocsp_req :: CertID < 'a > , pub cert_status : CertStatus , pub this_update : asn1 :: X509GeneralizedTime , # [explicit (0)] pub next_update : Option < asn1 :: X509GeneralizedTime > , # [explicit (1)] pub raw_single_extensions : Option < extensions :: RawExtensions < 'a > > , }
};
}
