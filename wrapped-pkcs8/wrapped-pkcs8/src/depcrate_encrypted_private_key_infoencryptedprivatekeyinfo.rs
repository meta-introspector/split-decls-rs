// Generated macro for EncryptedPrivateKeyInfo (struct)
macro_rules! Depcrate_encrypted_private_key_infoEncryptedPrivateKeyInfo {
() => {
// Module: crate::encrypted_private_key_info
// Provides: {"EncryptedPrivateKeyInfo"}
// Dependencies: {}
# [doc = " PKCS#8 `EncryptedPrivateKeyInfo`."] # [doc = ""] # [doc = " ASN.1 structure containing a PKCS#5 [`EncryptionScheme`] identifier for a"] # [doc = " password-based symmetric encryption scheme and encrypted private key data."] # [doc = ""] # [doc = " ## Schema"] # [doc = " Structure described in [RFC 5208 Section 6]:"] # [doc = ""] # [doc = " ```text"] # [doc = " EncryptedPrivateKeyInfo ::= SEQUENCE {"] # [doc = "   encryptionAlgorithm  EncryptionAlgorithmIdentifier,"] # [doc = "   encryptedData        EncryptedData }"] # [doc = ""] # [doc = " EncryptionAlgorithmIdentifier ::= AlgorithmIdentifier"] # [doc = ""] # [doc = " EncryptedData ::= OCTET STRING"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5208 Section 6]: https://tools.ietf.org/html/rfc5208#section-6"] # [derive (Clone , Eq , PartialEq)] pub struct EncryptedPrivateKeyInfo < Data > { # [doc = " Algorithm identifier describing a password-based symmetric encryption"] # [doc = " scheme used to encrypt the `encrypted_data` field."] pub encryption_algorithm : EncryptionScheme , # [doc = " Private key data"] pub encrypted_data : Data , }
};
}
