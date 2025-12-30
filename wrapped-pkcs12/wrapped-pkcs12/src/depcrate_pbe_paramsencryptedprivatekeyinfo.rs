// Generated macro for EncryptedPrivateKeyInfo (struct)
macro_rules! Depcrate_pbe_paramsEncryptedPrivateKeyInfo {
() => {
// Module: crate::pbe_params
// Provides: {"EncryptedPrivateKeyInfo"}
// Dependencies: {}
# [doc = " EncryptedPrivateKeyInfo ::= SEQUENCE {"] # [doc = "   encryptionAlgorithm  EncryptionAlgorithmIdentifier,"] # [doc = "   encryptedData        EncryptedData }"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct EncryptedPrivateKeyInfo { pub encryption_algorithm : AlgorithmIdentifierOwned , pub encrypted_data : OctetString , }
};
}
