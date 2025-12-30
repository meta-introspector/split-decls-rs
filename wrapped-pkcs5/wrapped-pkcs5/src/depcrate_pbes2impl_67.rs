// Generated macro for impl_67 (impl)
macro_rules! Depcrate_pbes2impl_67 {
() => {
// Module: crate::pbes2
// Provides: {"impl_67"}
// Dependencies: {}
impl Encode for EncryptionScheme { fn encoded_len (& self) -> der :: Result < Length > { AlgorithmIdentifierRef :: try_from (self) ? . encoded_len () } fn encode (& self , writer : & mut impl Writer) -> der :: Result < () > { AlgorithmIdentifierRef :: try_from (self) ? . encode (writer) } }
};
}
