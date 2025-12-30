// Generated macro for test_remainder (function)
macro_rules! Depcrate_de_teststest_remainder {
() => {
// Module: crate::de::tests
// Provides: {"test_remainder"}
// Dependencies: {}
# [test] fn test_remainder () { let mut deserializer = super :: Deserializer :: from_str ("  42  ") . unwrap () ; assert_eq ! (< u8 as serde :: Deserialize >:: deserialize (& mut deserializer) . unwrap () , 42) ; assert_eq ! (deserializer . remainder () , "  ") ; assert_eq ! (deserializer . end () , Ok (())) ; let mut deserializer = super :: Deserializer :: from_str ("  42 37 ") . unwrap () ; assert_eq ! (< u8 as serde :: Deserialize >:: deserialize (& mut deserializer) . unwrap () , 42) ; assert_eq ! (deserializer . remainder () , " 37 ") ; assert_eq ! (deserializer . end () , Err (Error :: TrailingCharacters)) ; }
};
}
