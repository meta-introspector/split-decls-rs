// Generated macro for impl_36 (impl)
macro_rules! Depcrate_paramsimpl_36 {
() => {
// Module: crate::params
// Provides: {"impl_36"}
// Dependencies: {}
impl EncodeValue for RsaOaepParams < '_ > { fn value_len (& self) -> der :: Result < Length > { self . context_specific_hash () . encoded_len () ? + self . context_specific_mask_gen () . encoded_len () ? + self . context_specific_p_source () . encoded_len () ? } fn encode_value (& self , writer : & mut impl Writer) -> der :: Result < () > { self . context_specific_hash () . encode (writer) ? ; self . context_specific_mask_gen () . encode (writer) ? ; self . context_specific_p_source () . encode (writer) ? ; Ok (()) } }
};
}
