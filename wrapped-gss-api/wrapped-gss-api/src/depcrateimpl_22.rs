// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl EncodeValue for InitialContextToken < '_ > { fn value_len (& self) -> der :: Result < Length > { self . this_mech . encoded_len () ? + self . inner_context_token . encoded_len () ? } fn encode_value (& self , writer : & mut impl Writer) -> der :: Result < () > { self . this_mech . encode (writer) ? ; self . inner_context_token . encode (writer) ? ; Ok (()) } }
};
}
