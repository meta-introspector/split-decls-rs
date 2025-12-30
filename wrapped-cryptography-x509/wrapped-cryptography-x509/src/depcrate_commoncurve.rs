// Generated macro for Curve (struct)
macro_rules! Depcrate_commonCurve {
() => {
// Module: crate::common
// Provides: {"Curve"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write , Hash , Clone , PartialEq , Eq , Debug)] pub struct Curve < 'a > { pub a : & 'a [u8] , pub b : & 'a [u8] , pub seed : Option < asn1 :: BitString < 'a > > , }
};
}
