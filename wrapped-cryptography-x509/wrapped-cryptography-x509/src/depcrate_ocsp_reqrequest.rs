// Generated macro for Request (struct)
macro_rules! Depcrate_ocsp_reqRequest {
() => {
// Module: crate::ocsp_req
// Provides: {"Request"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct Request < 'a > { pub req_cert : CertID < 'a > , # [explicit (0)] pub single_request_extensions : Option < extensions :: RawExtensions < 'a > > , }
};
}
