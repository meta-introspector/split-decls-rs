// Generated macro for test_unclosed_limited_seq_struct (function)
macro_rules! Depcrate_de_teststest_unclosed_limited_seq_struct {
() => {
// Module: crate::de::tests
// Provides: {"test_unclosed_limited_seq_struct"}
// Dependencies: {}
# [test] fn test_unclosed_limited_seq_struct () { # [derive (Debug , PartialEq)] struct LimitedStruct ; impl < 'de > serde :: Deserialize < 'de > for LimitedStruct { fn deserialize < D : serde :: Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { struct Visitor ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor { type Value = LimitedStruct ; fn expecting (& self , fmt : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { fmt . write_str ("struct LimitedStruct") } fn visit_map < A : serde :: de :: MapAccess < 'de > > (self , _map : A ,) -> Result < Self :: Value , A :: Error > { Ok (LimitedStruct) } } deserializer . deserialize_struct ("LimitedStruct" , & [] , Visitor) } } check_from_str_bytes_reader :: < LimitedStruct > ("(" , Err (SpannedError { code : Error :: ExpectedStructLikeEnd , span : Span { start : Position { line : 1 , col : 1 } , end : Position { line : 1 , col : 2 } , } , }) ,) }
};
}
