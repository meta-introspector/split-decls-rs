// Generated macro for SignatureParams (trait)
macro_rules! Depcrate_paramSignatureParams {
() => {
// Module: crate::param
// Provides: {"SignatureParams"}
// Dependencies: {}
pub trait SignatureParams : ParameterSet { type W1Size : ArraySize ; type ZSize : ArraySize ; type HintSize : ArraySize ; type SignatureSize : ArraySize ; const GAMMA1_MINUS_BETA : u32 ; const GAMMA2_MINUS_BETA : u32 ; fn split_hint (y : & EncodedHint < Self >) -> (& EncodedHintIndices < Self > , & EncodedHintCuts < Self >) ; fn encode_w1 (t1 : & Vector < Self :: K >) -> EncodedW1 < Self > ; fn decode_w1 (enc : & EncodedW1 < Self >) -> Vector < Self :: K > ; fn encode_z (z : & Vector < Self :: L >) -> EncodedZ < Self > ; fn decode_z (enc : & EncodedZ < Self >) -> Vector < Self :: L > ; fn concat_sig (c_tilde : EncodedCTilde < Self > , z : EncodedZ < Self > , h : EncodedHint < Self > ,) -> EncodedSignature < Self > ; fn split_sig (enc : & EncodedSignature < Self > ,) -> (& EncodedCTilde < Self > , & EncodedZ < Self > , & EncodedHint < Self >) ; }
};
}
