// Generated macro for impl_92 (impl)
macro_rules! Depcrateimpl_92 {
() => {
// Module: crate
// Provides: {"impl_92"}
// Dependencies: {}
impl EncodeValue for Signature { fn value_len (& self) -> der :: Result < Length > { UintRef :: new (& self . r . to_be_bytes ()) ? . encoded_len () ? + UintRef :: new (& self . s . to_be_bytes ()) ? . encoded_len () ? } fn encode_value (& self , writer : & mut impl Writer) -> der :: Result < () > { UintRef :: new (& self . r . to_be_bytes ()) ? . encode (writer) ? ; UintRef :: new (& self . s . to_be_bytes ()) ? . encode (writer) ? ; Ok (()) } }
};
}
