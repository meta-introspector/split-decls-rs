// Generated macro for impl_34 (impl)
macro_rules! Depcrate_pbes1impl_34 {
() => {
// Module: crate::pbes1
// Provides: {"impl_34"}
// Dependencies: {}
impl Encode for EncryptionScheme { fn encoded_len (& self) -> der :: Result < Length > { self . oid () . encoded_len () } fn encode (& self , writer : & mut impl Writer) -> der :: Result < () > { self . oid () . encode (writer) } }
};
}
