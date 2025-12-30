// Generated macro for DigestInfo (struct)
macro_rules! Depcrate_pkcs7DigestInfo {
() => {
// Module: crate::pkcs7
// Provides: {"DigestInfo"}
// Dependencies: {}
# [derive (asn1 :: Asn1Write , asn1 :: Asn1Read)] pub struct DigestInfo < 'a > { pub algorithm : common :: AlgorithmIdentifier < 'a > , pub digest : & 'a [u8] , }
};
}
