// Generated macro for CertStatus (enum)
macro_rules! Depcrate_ocsp_respCertStatus {
() => {
// Module: crate::ocsp_resp
// Provides: {"CertStatus"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub enum CertStatus { # [implicit (0)] Good (()) , # [implicit (1)] Revoked (RevokedInfo) , # [implicit (2)] Unknown (()) , }
};
}
