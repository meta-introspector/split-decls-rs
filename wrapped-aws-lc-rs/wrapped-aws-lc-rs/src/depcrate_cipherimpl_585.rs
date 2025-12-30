// Generated macro for impl_585 (impl)
macro_rules! Depcrate_cipherimpl_585 {
() => {
// Module: crate::cipher
// Provides: {"impl_585"}
// Dependencies: {}
impl TryInto < SymmetricCipherKey > for UnboundCipherKey { type Error = Unspecified ; fn try_into (self) -> Result < SymmetricCipherKey , Self :: Error > { match self . algorithm . id () { AlgorithmId :: Aes128 => SymmetricCipherKey :: aes128 (self . key_bytes . as_ref ()) , AlgorithmId :: Aes192 => SymmetricCipherKey :: aes192 (self . key_bytes . as_ref ()) , AlgorithmId :: Aes256 => SymmetricCipherKey :: aes256 (self . key_bytes . as_ref ()) , } } }
};
}
