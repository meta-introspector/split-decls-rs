// Generated macro for impl_66 (impl)
macro_rules! Depcrate_pbes2impl_66 {
() => {
// Module: crate::pbes2
// Provides: {"impl_66"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a EncryptionScheme > for AlgorithmIdentifierRef < 'a > { type Error = der :: Error ; fn try_from (scheme : & 'a EncryptionScheme) -> der :: Result < Self > { let parameters = OctetStringRef :: new (match scheme { EncryptionScheme :: Aes128Cbc { iv } => iv . as_slice () , EncryptionScheme :: Aes192Cbc { iv } => iv . as_slice () , EncryptionScheme :: Aes256Cbc { iv } => iv . as_slice () , EncryptionScheme :: Aes128Gcm { nonce } => nonce . as_slice () , EncryptionScheme :: Aes256Gcm { nonce } => nonce . as_slice () , # [cfg (feature = "des-insecure")] EncryptionScheme :: DesCbc { iv } => iv . as_slice () , # [cfg (feature = "3des")] EncryptionScheme :: DesEde3Cbc { iv } => iv . as_slice () , }) ? ; Ok (AlgorithmIdentifierRef { oid : scheme . oid () , parameters : Some (parameters . into ()) , }) } }
};
}
