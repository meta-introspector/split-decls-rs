// Generated macro for option_array (function)
macro_rules! Depcrate_serde_testsoption_array {
() => {
// Module: crate::serde_tests
// Provides: {"option_array"}
// Dependencies: {}
# [test] fn option_array () { let obj = vec ! [None , Some (None) , Some (Some (144))] ; let comparison = & [Event :: StartArray (Some (3)) , Event :: StartDictionary (Some (1)) , Event :: String ("None" . into ()) , Event :: String ("" . into ()) , Event :: EndCollection , Event :: StartDictionary (Some (1)) , Event :: String ("Some" . into ()) , Event :: StartDictionary (Some (1)) , Event :: String ("None" . into ()) , Event :: String ("" . into ()) , Event :: EndCollection , Event :: EndCollection , Event :: StartDictionary (Some (1)) , Event :: String ("Some" . into ()) , Event :: StartDictionary (Some (1)) , Event :: String ("Some" . into ()) , Event :: Integer (144 . into ()) , Event :: EndCollection , Event :: EndCollection , Event :: EndCollection ,] ; assert_roundtrip (obj , comparison , true) ; }
};
}
