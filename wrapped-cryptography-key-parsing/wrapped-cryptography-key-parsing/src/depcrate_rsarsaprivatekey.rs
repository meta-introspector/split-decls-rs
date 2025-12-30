// Generated macro for RsaPrivateKey (struct)
macro_rules! Depcrate_rsaRsaPrivateKey {
() => {
// Module: crate::rsa
// Provides: {"RsaPrivateKey"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub (crate) struct RsaPrivateKey < 'a > { pub (crate) version : u8 , pub (crate) n : asn1 :: BigUint < 'a > , pub (crate) e : asn1 :: BigUint < 'a > , pub (crate) d : asn1 :: BigUint < 'a > , pub (crate) p : asn1 :: BigUint < 'a > , pub (crate) q : asn1 :: BigUint < 'a > , pub (crate) dmp1 : asn1 :: BigUint < 'a > , pub (crate) dmq1 : asn1 :: BigUint < 'a > , pub (crate) iqmp : asn1 :: BigUint < 'a > , pub (crate) other_prime_infos : Option < asn1 :: SequenceOf < 'a , asn1 :: Sequence < 'a > , 1 > > , }
};
}
