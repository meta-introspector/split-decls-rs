// Generated macro for Content (enum)
macro_rules! Depcrate_pkcs7Content {
() => {
// Module: crate::pkcs7
// Provides: {"Content"}
// Dependencies: {}
# [allow (clippy :: large_enum_variant)] # [derive (asn1 :: Asn1DefinedByWrite , asn1 :: Asn1DefinedByRead)] pub enum Content < 'a > { # [defined_by (PKCS7_ENVELOPED_DATA_OID)] EnvelopedData (asn1 :: Explicit < Box < EnvelopedData < 'a > > , 0 >) , # [defined_by (PKCS7_SIGNED_DATA_OID)] SignedData (asn1 :: Explicit < Box < SignedData < 'a > > , 0 >) , # [defined_by (PKCS7_DATA_OID)] Data (Option < asn1 :: Explicit < & 'a [u8] , 0 > >) , # [defined_by (PKCS7_ENCRYPTED_DATA_OID)] EncryptedData (asn1 :: Explicit < EncryptedData < 'a > , 0 >) , }
};
}
