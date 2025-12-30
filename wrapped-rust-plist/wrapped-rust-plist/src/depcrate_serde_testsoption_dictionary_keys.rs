// Generated macro for option_dictionary_keys (function)
macro_rules! Depcrate_serde_testsoption_dictionary_keys {
() => {
// Module: crate::serde_tests
// Provides: {"option_dictionary_keys"}
// Dependencies: {}
# [test] fn option_dictionary_keys () { let mut obj = BTreeMap :: new () ; obj . insert (None , 1) ; obj . insert (Some (None) , 2) ; obj . insert (Some (Some (144)) , 3) ; let comparison = & [Event :: StartDictionary (Some (3)) , Event :: StartDictionary (Some (1)) , Event :: String ("None" . into ()) , Event :: String ("" . into ()) , Event :: EndCollection , Event :: Integer (1 . into ()) , Event :: StartDictionary (Some (1)) , Event :: String ("Some" . into ()) , Event :: StartDictionary (Some (1)) , Event :: String ("None" . into ()) , Event :: String ("" . into ()) , Event :: EndCollection , Event :: EndCollection , Event :: Integer (2 . into ()) , Event :: StartDictionary (Some (1)) , Event :: String ("Some" . into ()) , Event :: StartDictionary (Some (1)) , Event :: String ("Some" . into ()) , Event :: Integer (144 . into ()) , Event :: EndCollection , Event :: EndCollection , Event :: Integer (3 . into ()) , Event :: EndCollection ,] ; assert_roundtrip (obj , comparison , false) ; }
};
}
