// Generated macro for type_with_options (function)
macro_rules! Depcrate_serde_teststype_with_options {
() => {
// Module: crate::serde_tests
// Provides: {"type_with_options"}
// Dependencies: {}
# [test] fn type_with_options () { let inner = TypeWithOptions { a : None , b : Some (Some (12)) , c : None , } ; let obj = TypeWithOptions { a : Some ("hello" . to_owned ()) , b : Some (None) , c : Some (Box :: new (inner)) , } ; let comparison = & [Event :: StartDictionary (None) , Event :: String ("a" . into ()) , Event :: String ("hello" . into ()) , Event :: String ("b" . into ()) , Event :: StartDictionary (Some (1)) , Event :: String ("None" . into ()) , Event :: String ("" . into ()) , Event :: EndCollection , Event :: String ("c" . into ()) , Event :: StartDictionary (None) , Event :: String ("b" . into ()) , Event :: StartDictionary (Some (1)) , Event :: String ("Some" . into ()) , Event :: Integer (12 . into ()) , Event :: EndCollection , Event :: EndCollection , Event :: EndCollection ,] ; assert_roundtrip (obj , comparison , true) ; }
};
}
