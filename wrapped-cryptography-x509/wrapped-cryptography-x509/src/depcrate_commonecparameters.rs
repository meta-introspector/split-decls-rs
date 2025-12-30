// Generated macro for EcParameters (enum)
macro_rules! Depcrate_commonEcParameters {
() => {
// Module: crate::common
// Provides: {"EcParameters"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write , Hash , Clone , PartialEq , Eq , Debug)] pub enum EcParameters < 'a > { NamedCurve (asn1 :: ObjectIdentifier) , ImplicitCurve (asn1 :: Null) , SpecifiedCurve (SpecifiedECDomain < 'a >) , }
};
}
