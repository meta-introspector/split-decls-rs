// Generated macro for impl_22 (impl)
macro_rules! Depcrate_paramsimpl_22 {
() => {
// Module: crate::params
// Provides: {"impl_22"}
// Dependencies: {}
impl EncodeValue for TrailerField { fn value_len (& self) -> der :: Result < Length > { Ok (Length :: ONE) } fn encode_value (& self , writer : & mut impl Writer) -> der :: Result < () > { (* self as u8) . encode_value (writer) } }
};
}
