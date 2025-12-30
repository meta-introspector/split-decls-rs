// Generated macro for DHParams (struct)
macro_rules! Depcrate_commonDHParams {
() => {
// Module: crate::common
// Provides: {"DHParams"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct DHParams < 'a > { pub p : asn1 :: BigUint < 'a > , pub g : asn1 :: BigUint < 'a > , pub q : Option < asn1 :: BigUint < 'a > > , }
};
}
