// Generated macro for cat_with_firmware (function)
macro_rules! Depcrate_serde_testscat_with_firmware {
() => {
// Module: crate::serde_tests
// Provides: {"cat_with_firmware"}
// Dependencies: {}
# [test] fn cat_with_firmware () { let cat = Animal :: Cat { age : 12 . into () , name : "Paws" . to_owned () , firmware : Some (vec ! [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8]) , } ; let comparison = & [Event :: StartDictionary (Some (1)) , Event :: String ("Cat" . into ()) , Event :: StartDictionary (None) , Event :: String ("age" . into ()) , Event :: Integer (12 . into ()) , Event :: String ("name" . into ()) , Event :: String ("Paws" . into ()) , Event :: String ("firmware" . into ()) , Event :: StartArray (Some (9)) , Event :: Integer (0 . into ()) , Event :: Integer (1 . into ()) , Event :: Integer (2 . into ()) , Event :: Integer (3 . into ()) , Event :: Integer (4 . into ()) , Event :: Integer (5 . into ()) , Event :: Integer (6 . into ()) , Event :: Integer (7 . into ()) , Event :: Integer (8 . into ()) , Event :: EndCollection , Event :: EndCollection , Event :: EndCollection ,] ; assert_roundtrip (cat , comparison , true) ; }
};
}
