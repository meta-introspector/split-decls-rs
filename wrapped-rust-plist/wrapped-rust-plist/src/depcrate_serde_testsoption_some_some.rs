// Generated macro for option_some_some (function)
macro_rules! Depcrate_serde_testsoption_some_some {
() => {
// Module: crate::serde_tests
// Provides: {"option_some_some"}
// Dependencies: {}
# [test] fn option_some_some () { let obj = Some (Some (12)) ; let comparison = & [Event :: StartDictionary (Some (1)) , Event :: String ("Some" . into ()) , Event :: Integer (12 . into ()) , Event :: EndCollection ,] ; assert_roundtrip (obj , comparison , true) ; }
};
}
