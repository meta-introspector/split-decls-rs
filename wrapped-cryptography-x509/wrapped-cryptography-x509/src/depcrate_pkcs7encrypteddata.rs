// Generated macro for EncryptedData (struct)
macro_rules! Depcrate_pkcs7EncryptedData {
() => {
// Module: crate::pkcs7
// Provides: {"EncryptedData"}
// Dependencies: {}
# [derive (asn1 :: Asn1Write , asn1 :: Asn1Read)] pub struct EncryptedData < 'a > { pub version : u8 , pub encrypted_content_info : EncryptedContentInfo < 'a > , }
};
}
