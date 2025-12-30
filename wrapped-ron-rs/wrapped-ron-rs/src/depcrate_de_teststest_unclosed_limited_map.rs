// Generated macro for test_unclosed_limited_map (function)
macro_rules! Depcrate_de_teststest_unclosed_limited_map {
() => {
// Module: crate::de::tests
// Provides: {"test_unclosed_limited_map"}
// Dependencies: {}
# [test] fn test_unclosed_limited_map () { # [derive (Debug , PartialEq)] struct LimitedMap ; impl < 'de > serde :: Deserialize < 'de > for LimitedMap { fn deserialize < D : serde :: Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { struct Visitor ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor { type Value = LimitedMap ; fn expecting (& self , fmt : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { fmt . write_str ("an empty map") } fn visit_map < A : serde :: de :: MapAccess < 'de > > (self , _map : A ,) -> Result < Self :: Value , A :: Error > { Ok (LimitedMap) } } deserializer . deserialize_map (Visitor) } } check_from_str_bytes_reader :: < LimitedMap > ("{" , Err (SpannedError { code : Error :: ExpectedMapEnd , span : Span { start : Position { line : 1 , col : 1 } , end : Position { line : 1 , col : 2 } , } , }) ,) ; assert_eq ! (crate :: Value :: Map ([("a" , 42)] . into_iter () . collect ()) . into_rust ::< LimitedMap > () , Err (Error :: ExpectedDifferentLength { expected : String :: from ("a map of length 0") , found : 1 })) ; }
};
}
