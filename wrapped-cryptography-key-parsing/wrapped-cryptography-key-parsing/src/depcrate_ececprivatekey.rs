// Generated macro for EcPrivateKey (struct)
macro_rules! Depcrate_ecEcPrivateKey {
() => {
// Module: crate::ec
// Provides: {"EcPrivateKey"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub (crate) struct EcPrivateKey < 'a > { pub (crate) version : u8 , pub (crate) private_key : & 'a [u8] , # [explicit (0)] pub (crate) parameters : Option < EcParameters < 'a > > , # [explicit (1)] pub (crate) public_key : Option < asn1 :: BitString < 'a > > , }
};
}
