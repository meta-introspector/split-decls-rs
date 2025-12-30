// Generated macro for impl_82 (impl)
macro_rules! Depcrate_encrypted_private_key_infoimpl_82 {
() => {
// Module: crate::encrypted_private_key_info
// Provides: {"impl_82"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a , Data > TryFrom < & EncryptedPrivateKeyInfo < Data > > for SecretDocument where Data : DecodeValue < 'a , Error = der :: Error > + EncodeValue + FixedTag + 'a , { type Error = Error ; fn try_from (encrypted_private_key : & EncryptedPrivateKeyInfo < Data >) -> Result < SecretDocument > { Ok (Self :: encode_msg (encrypted_private_key) ?) } }
};
}
