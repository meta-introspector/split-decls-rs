// Generated macro for ResponseBytes (struct)
macro_rules! Depcrate_ocsp_respResponseBytes {
() => {
// Module: crate::ocsp_resp
// Provides: {"ResponseBytes"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct ResponseBytes < 'a > { pub response_type : asn1 :: ObjectIdentifier , pub response : asn1 :: OctetStringEncoded < BasicOCSPResponse < 'a > > , }
};
}
