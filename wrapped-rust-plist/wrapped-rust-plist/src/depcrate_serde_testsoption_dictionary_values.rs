// Generated macro for option_dictionary_values (function)
macro_rules! Depcrate_serde_testsoption_dictionary_values {
() => {
// Module: crate::serde_tests
// Provides: {"option_dictionary_values"}
// Dependencies: {}
# [test] fn option_dictionary_values () { let mut obj = BTreeMap :: new () ; obj . insert ("a" . to_owned () , None) ; obj . insert ("b" . to_owned () , Some (None)) ; obj . insert ("c" . to_owned () , Some (Some (144))) ; let comparison = & [Event :: StartDictionary (Some (3)) , Event :: String ("a" . into ()) , Event :: StartDictionary (Some (1)) , Event :: String ("None" . into ()) , Event :: String ("" . into ()) , Event :: EndCollection , Event :: String ("b" . into ()) , Event :: StartDictionary (Some (1)) , Event :: String ("Some" . into ()) , Event :: StartDictionary (Some (1)) , Event :: String ("None" . into ()) , Event :: String ("" . into ()) , Event :: EndCollection , Event :: EndCollection , Event :: String ("c" . into ()) , Event :: StartDictionary (Some (1)) , Event :: String ("Some" . into ()) , Event :: StartDictionary (Some (1)) , Event :: String ("Some" . into ()) , Event :: Integer (144 . into ()) , Event :: EndCollection , Event :: EndCollection , Event :: EndCollection ,] ; assert_roundtrip (obj , comparison , true) ; }
};
}
