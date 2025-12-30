// Generated macro for SpecifiedECDomain (struct)
macro_rules! Depcrate_commonSpecifiedECDomain {
() => {
// Module: crate::common
// Provides: {"SpecifiedECDomain"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write , Hash , Clone , Eq , PartialEq , Debug)] pub struct SpecifiedECDomain < 'a > { pub version : u8 , pub field_id : FieldID < 'a > , pub curve : Curve < 'a > , pub base : & 'a [u8] , pub order : asn1 :: BigUint < 'a > , pub cofactor : Option < u8 > , }
};
}
