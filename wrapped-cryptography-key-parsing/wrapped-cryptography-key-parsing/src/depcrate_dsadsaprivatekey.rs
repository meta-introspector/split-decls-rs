// Generated macro for DsaPrivateKey (struct)
macro_rules! Depcrate_dsaDsaPrivateKey {
() => {
// Module: crate::dsa
// Provides: {"DsaPrivateKey"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] struct DsaPrivateKey < 'a > { version : u8 , p : asn1 :: BigUint < 'a > , q : asn1 :: BigUint < 'a > , g : asn1 :: BigUint < 'a > , pub_key : asn1 :: BigUint < 'a > , priv_key : asn1 :: BigUint < 'a > , }
};
}
