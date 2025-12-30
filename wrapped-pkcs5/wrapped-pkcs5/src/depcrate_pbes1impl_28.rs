// Generated macro for impl_28 (impl)
macro_rules! Depcrate_pbes1impl_28 {
() => {
// Module: crate::pbes1
// Provides: {"impl_28"}
// Dependencies: {}
impl EncodeValue for Parameters { fn value_len (& self) -> der :: Result < Length > { OctetStringRef :: new (& self . salt) ? . encoded_len () ? + self . iteration_count . encoded_len () ? } fn encode_value (& self , writer : & mut impl Writer) -> der :: Result < () > { OctetStringRef :: new (& self . salt) ? . encode (writer) ? ; self . iteration_count . encode (writer) ? ; Ok (()) } }
};
}
