// Generated macro for impl_59 (impl)
macro_rules! Depcrate_pbes2impl_59 {
() => {
// Module: crate::pbes2
// Provides: {"impl_59"}
// Dependencies: {}
impl EncodeValue for Parameters { fn value_len (& self) -> der :: Result < Length > { self . kdf . encoded_len () ? + self . encryption . encoded_len () ? } fn encode_value (& self , writer : & mut impl Writer) -> der :: Result < () > { self . kdf . encode (writer) ? ; self . encryption . encode (writer) ? ; Ok (()) } }
};
}
