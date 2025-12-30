// Generated macro for ResponseData (struct)
macro_rules! Depcrate_ocsp_respResponseData {
() => {
// Module: crate::ocsp_resp
// Provides: {"ResponseData"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct ResponseData < 'a > { # [explicit (0)] # [default (0)] pub version : u8 , pub responder_id : ResponderId < 'a > , pub produced_at : asn1 :: X509GeneralizedTime , pub responses : common :: Asn1ReadableOrWritable < asn1 :: SequenceOf < 'a , SingleResponse < 'a > > , asn1 :: SequenceOfWriter < 'a , SingleResponse < 'a > , Vec < SingleResponse < 'a > > > , > , # [explicit (1)] pub raw_response_extensions : Option < extensions :: RawExtensions < 'a > > , }
};
}
