// Generated macro for BasicOCSPResponse (struct)
macro_rules! Depcrate_ocsp_respBasicOCSPResponse {
() => {
// Module: crate::ocsp_resp
// Provides: {"BasicOCSPResponse"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct BasicOCSPResponse < 'a > { pub tbs_response_data : ResponseData < 'a > , pub signature_algorithm : common :: AlgorithmIdentifier < 'a > , pub signature : asn1 :: BitString < 'a > , # [explicit (0)] pub certs : OCSPCerts < 'a > , }
};
}
