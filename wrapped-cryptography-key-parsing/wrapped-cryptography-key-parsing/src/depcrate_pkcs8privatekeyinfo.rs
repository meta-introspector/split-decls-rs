// Generated macro for PrivateKeyInfo (struct)
macro_rules! Depcrate_pkcs8PrivateKeyInfo {
() => {
// Module: crate::pkcs8
// Provides: {"PrivateKeyInfo"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct PrivateKeyInfo < 'a > { pub version : u8 , pub algorithm : AlgorithmIdentifier < 'a > , pub private_key : & 'a [u8] , # [implicit (0)] pub attributes : Option < Attributes < 'a > > , }
};
}
