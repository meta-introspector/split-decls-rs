// Generated macro for impl_23 (impl)
macro_rules! Depcrate_pbes1impl_23 {
() => {
// Module: crate::pbes1
// Provides: {"impl_23"}
// Dependencies: {}
impl EncodeValue for Algorithm { fn value_len (& self) -> der :: Result < Length > { self . encryption . encoded_len () ? + self . parameters . encoded_len () ? } fn encode_value (& self , writer : & mut impl Writer) -> der :: Result < () > { self . encryption . encode (writer) ? ; self . parameters . encode (writer) ? ; Ok (()) } }
};
}
