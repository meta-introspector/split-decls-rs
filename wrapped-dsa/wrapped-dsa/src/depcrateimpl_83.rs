// Generated macro for impl_83 (impl)
macro_rules! Depcrateimpl_83 {
() => {
// Module: crate
// Provides: {"impl_83"}
// Dependencies: {}
impl EncodeValue for Signature { fn value_len (& self) -> der :: Result < Length > { UintRef :: new (& self . r . to_be_bytes ()) ? . encoded_len () ? + UintRef :: new (& self . s . to_be_bytes ()) ? . encoded_len () ? } fn encode_value (& self , writer : & mut impl Writer) -> der :: Result < () > { UintRef :: new (& self . r . to_be_bytes ()) ? . encode (writer) ? ; UintRef :: new (& self . s . to_be_bytes ()) ? . encode (writer) ? ; Ok (()) } }
};
}
