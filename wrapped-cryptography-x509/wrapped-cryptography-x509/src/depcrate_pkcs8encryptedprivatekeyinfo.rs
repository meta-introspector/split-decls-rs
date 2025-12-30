// Generated macro for EncryptedPrivateKeyInfo (struct)
macro_rules! Depcrate_pkcs8EncryptedPrivateKeyInfo {
() => {
// Module: crate::pkcs8
// Provides: {"EncryptedPrivateKeyInfo"}
// Dependencies: {}
# [derive (asn1 :: Asn1Write , asn1 :: Asn1Read)] pub struct EncryptedPrivateKeyInfo < 'a > { pub encryption_algorithm : AlgorithmIdentifier < 'a > , pub encrypted_data : & 'a [u8] , }
};
}
