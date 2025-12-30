// Generated macro for RecipientInfo (struct)
macro_rules! Depcrate_pkcs7RecipientInfo {
() => {
// Module: crate::pkcs7
// Provides: {"RecipientInfo"}
// Dependencies: {}
# [derive (asn1 :: Asn1Write , asn1 :: Asn1Read)] pub struct RecipientInfo < 'a > { pub version : u8 , pub issuer_and_serial_number : IssuerAndSerialNumber < 'a > , pub key_encryption_algorithm : common :: AlgorithmIdentifier < 'a > , pub encrypted_key : & 'a [u8] , }
};
}
