// Generated macro for Time (enum)
macro_rules! Depcrate_commonTime {
() => {
// Module: crate::common
// Provides: {"Time"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write , PartialEq , Eq , Hash , Clone)] pub enum Time { UtcTime (asn1 :: UtcTime) , GeneralizedTime (asn1 :: X509GeneralizedTime) , }
};
}
