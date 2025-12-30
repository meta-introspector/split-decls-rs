// Generated macro for VerifyingKeyParams (trait)
macro_rules! Depcrate_paramVerifyingKeyParams {
() => {
// Module: crate::param
// Provides: {"VerifyingKeyParams"}
// Dependencies: {}
pub trait VerifyingKeyParams : ParameterSet { type T1Size : ArraySize ; type VerifyingKeySize : ArraySize ; fn encode_t1 (t1 : & Vector < Self :: K >) -> EncodedT1 < Self > ; fn decode_t1 (enc : & EncodedT1 < Self >) -> Vector < Self :: K > ; fn concat_vk (rho : B32 , t1 : EncodedT1 < Self >) -> EncodedVerifyingKey < Self > ; fn split_vk (enc : & EncodedVerifyingKey < Self >) -> (& B32 , & EncodedT1 < Self >) ; }
};
}
