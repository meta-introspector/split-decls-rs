// Generated macro for frog (function)
macro_rules! Depcrate_serde_testsfrog {
() => {
// Module: crate::serde_tests
// Provides: {"frog"}
// Dependencies: {}
# [test] fn frog () { let frog = Animal :: Frog (Ok ("hello" . to_owned ()) , Some (vec ! [1.0 , 2.0 , std :: f64 :: consts :: PI , 0.000000001 , 1.27e31]) ,) ; let comparison = & [Event :: StartDictionary (Some (1)) , Event :: String ("Frog" . into ()) , Event :: StartArray (Some (2)) , Event :: StartDictionary (Some (1)) , Event :: String ("Ok" . into ()) , Event :: String ("hello" . into ()) , Event :: EndCollection , Event :: StartDictionary (Some (1)) , Event :: String ("Some" . into ()) , Event :: StartArray (Some (5)) , Event :: Real (1.0) , Event :: Real (2.0) , Event :: Real (std :: f64 :: consts :: PI) , Event :: Real (0.000000001) , Event :: Real (1.27e31) , Event :: EndCollection , Event :: EndCollection , Event :: EndCollection , Event :: EndCollection ,] ; assert_roundtrip (frog , comparison , true) ; }
};
}
