// Generated macro for EncryptedContentInfo (struct)
macro_rules! Depcrate_pkcs7EncryptedContentInfo {
() => {
// Module: crate::pkcs7
// Provides: {"EncryptedContentInfo"}
// Dependencies: {}
# [derive (asn1 :: Asn1Write , asn1 :: Asn1Read)] pub struct EncryptedContentInfo < 'a > { pub content_type : asn1 :: ObjectIdentifier , pub content_encryption_algorithm : common :: AlgorithmIdentifier < 'a > , # [implicit (0)] pub encrypted_content : Option < & 'a [u8] > , }
};
}
