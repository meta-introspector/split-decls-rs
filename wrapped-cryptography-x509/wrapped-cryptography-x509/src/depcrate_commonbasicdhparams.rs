// Generated macro for BasicDHParams (struct)
macro_rules! Depcrate_commonBasicDHParams {
() => {
// Module: crate::common
// Provides: {"BasicDHParams"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write , Clone , PartialEq , Eq , Debug , Hash)] pub struct BasicDHParams < 'a > { pub p : asn1 :: BigUint < 'a > , pub g : asn1 :: BigUint < 'a > , pub private_value_length : Option < u32 > , }
};
}
