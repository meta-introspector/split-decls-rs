// Generated macro for TBSRequest (struct)
macro_rules! Depcrate_ocsp_reqTBSRequest {
() => {
// Module: crate::ocsp_req
// Provides: {"TBSRequest"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct TBSRequest < 'a > { # [explicit (0)] # [default (0)] pub version : u8 , # [explicit (1)] pub requestor_name : Option < name :: GeneralName < 'a > > , pub request_list : common :: Asn1ReadableOrWritable < asn1 :: SequenceOf < 'a , Request < 'a > > , asn1 :: SequenceOfWriter < 'a , Request < 'a > > , > , # [explicit (2)] pub raw_request_extensions : Option < extensions :: RawExtensions < 'a > > , }
};
}
