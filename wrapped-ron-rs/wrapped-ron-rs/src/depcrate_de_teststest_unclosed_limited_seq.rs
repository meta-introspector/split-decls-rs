// Generated macro for test_unclosed_limited_seq (function)
macro_rules! Depcrate_de_teststest_unclosed_limited_seq {
() => {
// Module: crate::de::tests
// Provides: {"test_unclosed_limited_seq"}
// Dependencies: {}
# [test] fn test_unclosed_limited_seq () { # [derive (Debug , PartialEq)] struct LimitedSeq ; impl < 'de > serde :: Deserialize < 'de > for LimitedSeq { fn deserialize < D : serde :: Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { struct Visitor ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor { type Value = LimitedSeq ; fn expecting (& self , fmt : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { fmt . write_str ("an empty sequence") } fn visit_seq < A : serde :: de :: SeqAccess < 'de > > (self , _seq : A ,) -> Result < Self :: Value , A :: Error > { Ok (LimitedSeq) } } deserializer . deserialize_seq (Visitor) } } check_from_str_bytes_reader :: < LimitedSeq > ("[" , Err (SpannedError { code : Error :: ExpectedArrayEnd , span : Span { start : Position { line : 1 , col : 1 } , end : Position { line : 1 , col : 2 } , } , }) ,) ; assert_eq ! (crate :: Value :: from (vec ! [42]) . into_rust ::< LimitedSeq > () , Err (Error :: ExpectedDifferentLength { expected : String :: from ("a sequence of length 0") , found : 1 })) ; }
};
}
