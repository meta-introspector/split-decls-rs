// Generated macro for impl_16 (impl)
macro_rules! Depcrate_componentsimpl_16 {
() => {
// Module: crate::components
// Provides: {"impl_16"}
// Dependencies: {}
impl EncodeValue for Components { fn value_len (& self) -> der :: Result < Length > { UintRef :: new (& self . p . to_be_bytes ()) ? . encoded_len () ? + UintRef :: new (& self . q . to_be_bytes ()) ? . encoded_len () ? + UintRef :: new (& self . g . to_be_bytes ()) ? . encoded_len () ? } fn encode_value (& self , writer : & mut impl Writer) -> der :: Result < () > { UintRef :: new (& self . p . to_be_bytes ()) ? . encode (writer) ? ; UintRef :: new (& self . q . to_be_bytes ()) ? . encode (writer) ? ; UintRef :: new (& self . g . to_be_bytes ()) ? . encode (writer) ? ; Ok (()) } }
};
}
