// Generated macro for KeyEncryptionInfo (enum)
macro_rules! Depcrate_builderKeyEncryptionInfo {
() => {
// Module: crate::builder
// Provides: {"KeyEncryptionInfo"}
// Dependencies: {}
# [doc = " Contains information required to encrypt the content encryption key with a specific method"] # [derive (Clone , Debug , Eq , PartialEq)] pub enum KeyEncryptionInfo { # [doc = " Encrypt key with RSA"] Rsa (rsa :: RsaPublicKey) , }
};
}
