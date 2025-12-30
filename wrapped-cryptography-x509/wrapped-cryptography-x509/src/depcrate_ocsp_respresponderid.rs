// Generated macro for ResponderId (enum)
macro_rules! Depcrate_ocsp_respResponderId {
() => {
// Module: crate::ocsp_resp
// Provides: {"ResponderId"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub enum ResponderId < 'a > { # [explicit (1)] ByName (name :: Name < 'a >) , # [explicit (2)] ByKey (& 'a [u8]) , }
};
}
