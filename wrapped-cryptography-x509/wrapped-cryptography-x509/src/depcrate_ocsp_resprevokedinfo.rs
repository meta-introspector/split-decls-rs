// Generated macro for RevokedInfo (struct)
macro_rules! Depcrate_ocsp_respRevokedInfo {
() => {
// Module: crate::ocsp_resp
// Provides: {"RevokedInfo"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct RevokedInfo { pub revocation_time : asn1 :: X509GeneralizedTime , # [explicit (0)] pub revocation_reason : Option < crl :: CRLReason > , }
};
}
