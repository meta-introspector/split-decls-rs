// Generated macro for SigningKeyParams (trait)
macro_rules! Depcrate_paramSigningKeyParams {
() => {
// Module: crate::param
// Provides: {"SigningKeyParams"}
// Dependencies: {}
pub trait SigningKeyParams : ParameterSet { type S1Size : ArraySize ; type S2Size : ArraySize ; type T0Size : ArraySize ; type SigningKeySize : ArraySize ; fn encode_s1 (s1 : & Vector < Self :: L >) -> EncodedS1 < Self > ; fn decode_s1 (enc : & EncodedS1 < Self >) -> Vector < Self :: L > ; fn encode_s2 (s2 : & Vector < Self :: K >) -> EncodedS2 < Self > ; fn decode_s2 (enc : & EncodedS2 < Self >) -> Vector < Self :: K > ; fn encode_t0 (t0 : & Vector < Self :: K >) -> EncodedT0 < Self > ; fn decode_t0 (enc : & EncodedT0 < Self >) -> Vector < Self :: K > ; fn concat_sk (rho : B32 , K : B32 , tr : B64 , s1 : EncodedS1 < Self > , s2 : EncodedS2 < Self > , t0 : EncodedT0 < Self > ,) -> EncodedSigningKey < Self > ; fn split_sk (enc : & EncodedSigningKey < Self > ,) -> (& B32 , & B32 , & B64 , & EncodedS1 < Self > , & EncodedS2 < Self > , & EncodedT0 < Self > ,) ; }
};
}
