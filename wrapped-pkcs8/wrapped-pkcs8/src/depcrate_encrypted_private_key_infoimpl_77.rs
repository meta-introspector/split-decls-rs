// Generated macro for impl_77 (impl)
macro_rules! Depcrate_encrypted_private_key_infoimpl_77 {
() => {
// Module: crate::encrypted_private_key_info
// Provides: {"impl_77"}
// Dependencies: {}
impl < Data > EncodeValue for EncryptedPrivateKeyInfo < Data > where Data : EncodeValue + FixedTag , { fn value_len (& self) -> der :: Result < Length > { self . encryption_algorithm . encoded_len () ? + self . encrypted_data . encoded_len () ? } fn encode_value (& self , writer : & mut impl Writer) -> der :: Result < () > { self . encryption_algorithm . encode (writer) ? ; self . encrypted_data . encode (writer) ? ; Ok (()) } }
};
}
