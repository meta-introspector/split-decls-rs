// Generated macro for DHXParams (struct)
macro_rules! Depcrate_commonDHXParams {
() => {
// Module: crate::common
// Provides: {"DHXParams"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write , Clone , PartialEq , Eq , Debug , Hash)] pub struct DHXParams < 'a > { pub p : asn1 :: BigUint < 'a > , pub g : asn1 :: BigUint < 'a > , pub q : asn1 :: BigUint < 'a > , pub j : Option < asn1 :: BigUint < 'a > > , pub validation_params : Option < asn1 :: Sequence < 'a > > , }
};
}
