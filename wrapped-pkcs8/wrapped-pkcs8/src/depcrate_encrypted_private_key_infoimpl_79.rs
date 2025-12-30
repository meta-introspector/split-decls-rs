// Generated macro for impl_79 (impl)
macro_rules! Depcrate_encrypted_private_key_infoimpl_79 {
() => {
// Module: crate::encrypted_private_key_info
// Provides: {"impl_79"}
// Dependencies: {}
impl < 'a , Data > TryFrom < & 'a [u8] > for EncryptedPrivateKeyInfo < Data > where Data : DecodeValue < 'a , Error = der :: Error > + EncodeValue + FixedTag + 'a , { type Error = Error ; fn try_from (bytes : & 'a [u8]) -> Result < Self > { Ok (Self :: from_der (bytes) ?) } }
};
}
