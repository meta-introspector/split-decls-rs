// Generated macro for dog (function)
macro_rules! Depcrate_serde_testsdog {
() => {
// Module: crate::serde_tests
// Provides: {"dog"}
// Dependencies: {}
# [test] fn dog () { let dog = Animal :: Dog (DogOuter { inner : vec ! [DogInner { a : () , b : 12 , c : vec ! ["a" . to_string () , "b" . to_string ()] , d : Some (Uid :: new (42)) , e : Data :: new (vec ! [20 , 22]) , }] , }) ; let comparison = & [Event :: StartDictionary (Some (1)) , Event :: String ("Dog" . into ()) , Event :: StartDictionary (None) , Event :: String ("inner" . into ()) , Event :: StartArray (Some (1)) , Event :: StartDictionary (None) , Event :: String ("a" . into ()) , Event :: String ("" . into ()) , Event :: String ("b" . into ()) , Event :: Integer (12 . into ()) , Event :: String ("c" . into ()) , Event :: StartArray (Some (2)) , Event :: String ("a" . into ()) , Event :: String ("b" . into ()) , Event :: EndCollection , Event :: String ("d" . into ()) , Event :: Uid (Uid :: new (42)) , Event :: String ("e" . into ()) , Event :: Data (vec ! [20 , 22] . into ()) , Event :: EndCollection , Event :: EndCollection , Event :: EndCollection , Event :: EndCollection ,] ; assert_roundtrip (dog , comparison , true) ; }
};
}
