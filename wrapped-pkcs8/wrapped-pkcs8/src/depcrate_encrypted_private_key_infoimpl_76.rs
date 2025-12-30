// Generated macro for impl_76 (impl)
macro_rules! Depcrate_encrypted_private_key_infoimpl_76 {
() => {
// Module: crate::encrypted_private_key_info
// Provides: {"impl_76"}
// Dependencies: {}
impl < 'a , Data > DecodeValue < 'a > for EncryptedPrivateKeyInfo < Data > where Data : DecodeValue < 'a , Error = der :: Error > + FixedTag + 'a , { type Error = der :: Error ; fn decode_value < R : Reader < 'a > > (reader : & mut R , _header : Header) -> der :: Result < Self > { Ok (Self { encryption_algorithm : reader . decode () ? , encrypted_data : reader . decode () ? , }) } }
};
}
