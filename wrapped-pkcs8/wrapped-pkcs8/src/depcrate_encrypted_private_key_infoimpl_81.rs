// Generated macro for impl_81 (impl)
macro_rules! Depcrate_encrypted_private_key_infoimpl_81 {
() => {
// Module: crate::encrypted_private_key_info
// Provides: {"impl_81"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a , Data > TryFrom < EncryptedPrivateKeyInfo < Data > > for SecretDocument where Data : DecodeValue < 'a , Error = der :: Error > + EncodeValue + FixedTag + 'a , { type Error = Error ; fn try_from (encrypted_private_key : EncryptedPrivateKeyInfo < Data >) -> Result < SecretDocument > { SecretDocument :: try_from (& encrypted_private_key) } }
};
}
