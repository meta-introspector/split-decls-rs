// Generated macro for impl_80 (impl)
macro_rules! Depcrate_encrypted_private_key_infoimpl_80 {
() => {
// Module: crate::encrypted_private_key_info
// Provides: {"impl_80"}
// Dependencies: {}
impl < Data > fmt :: Debug for EncryptedPrivateKeyInfo < Data > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("EncryptedPrivateKeyInfo") . field ("encryption_algorithm" , & self . encryption_algorithm) . finish_non_exhaustive () } }
};
}
