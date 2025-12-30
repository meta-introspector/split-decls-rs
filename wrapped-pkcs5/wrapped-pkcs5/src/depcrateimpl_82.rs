// Generated macro for impl_82 (impl)
macro_rules! Depcrateimpl_82 {
() => {
// Module: crate
// Provides: {"impl_82"}
// Dependencies: {}
impl TryFrom < & [u8] > for EncryptionScheme { type Error = der :: Error ; fn try_from (bytes : & [u8]) -> der :: Result < EncryptionScheme > { AlgorithmIdentifierRef :: from_der (bytes) ? . try_into () } }
};
}
