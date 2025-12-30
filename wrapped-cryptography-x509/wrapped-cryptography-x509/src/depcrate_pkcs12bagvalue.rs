// Generated macro for BagValue (enum)
macro_rules! Depcrate_pkcs12BagValue {
() => {
// Module: crate::pkcs12
// Provides: {"BagValue"}
// Dependencies: {}
# [derive (asn1 :: Asn1DefinedByWrite)] pub enum BagValue < 'a > { # [defined_by (CERT_BAG_OID)] CertBag (Box < CertBag < 'a > >) , # [defined_by (KEY_BAG_OID)] KeyBag (asn1 :: Tlv < 'a >) , # [defined_by (SHROUDED_KEY_BAG_OID)] ShroudedKeyBag (pkcs8 :: EncryptedPrivateKeyInfo < 'a >) , }
};
}
