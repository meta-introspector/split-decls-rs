// Generated macro for impl_28 (impl)
macro_rules! Depcrate_paramsimpl_28 {
() => {
// Module: crate::params
// Provides: {"impl_28"}
// Dependencies: {}
impl EncodeValue for RsaPssParams < '_ > { fn value_len (& self) -> der :: Result < Length > { self . context_specific_hash () . encoded_len () ? + self . context_specific_mask_gen () . encoded_len () ? + self . context_specific_salt_len () . encoded_len () ? + self . context_specific_trailer_field () . encoded_len () ? } fn encode_value (& self , writer : & mut impl Writer) -> der :: Result < () > { self . context_specific_hash () . encode (writer) ? ; self . context_specific_mask_gen () . encode (writer) ? ; self . context_specific_salt_len () . encode (writer) ? ; self . context_specific_trailer_field () . encode (writer) ? ; Ok (()) } }
};
}
