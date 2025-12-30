// Generated macro for impl_21 (impl)
macro_rules! Depcrate_paramsimpl_21 {
() => {
// Module: crate::params
// Provides: {"impl_21"}
// Dependencies: {}
impl < 'a > DecodeValue < 'a > for TrailerField { type Error = der :: Error ; fn decode_value < R : Reader < 'a > > (reader : & mut R , header : der :: Header) -> der :: Result < Self > { match u8 :: decode_value (reader , header) ? { 1 => Ok (TrailerField :: BC) , _ => Err (reader . error (Self :: TAG . value_error ())) , } } }
};
}
