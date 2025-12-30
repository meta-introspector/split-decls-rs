// Generated macro for impl_108 (impl)
macro_rules! Depcrate_paramimpl_108 {
() => {
// Module: crate::param
// Provides: {"impl_108"}
// Dependencies: {}
impl < P > VerifyingKeyParams for P where P : ParameterSet , U320 : Mul < P :: K > , Prod < U320 , P :: K > : ArraySize + Div < P :: K , Output = U320 > + Rem < P :: K , Output = U0 > , U32 : Add < Prod < U320 , P :: K > > , Sum < U32 , U32 > : ArraySize , Sum < U32 , Prod < U320 , P :: K > > : ArraySize + Sub < U32 , Output = Prod < U320 , P :: K > > , { type T1Size = EncodedVectorSize < BitlenQMinusD , P :: K > ; type VerifyingKeySize = Sum < U32 , Self :: T1Size > ; fn encode_t1 (t1 : & Vector < P :: K >) -> EncodedT1 < Self > { Encode :: < BitlenQMinusD > :: encode (t1) } fn decode_t1 (enc : & EncodedT1 < Self >) -> Vector < Self :: K > { Encode :: < BitlenQMinusD > :: decode (enc) } fn concat_vk (rho : B32 , t1 : EncodedT1 < Self >) -> EncodedVerifyingKey < Self > { rho . concat (t1) } fn split_vk (enc : & EncodedVerifyingKey < Self >) -> (& B32 , & EncodedT1 < Self >) { enc . split_ref () } }
};
}
