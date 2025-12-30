// Generated macro for cat_without_firmware (function)
macro_rules! Depcrate_serde_testscat_without_firmware {
() => {
// Module: crate::serde_tests
// Provides: {"cat_without_firmware"}
// Dependencies: {}
# [test] fn cat_without_firmware () { let cat = Animal :: Cat { age : Integer :: from (- 12) , name : "Paws" . to_owned () , firmware : None , } ; let comparison = & [Event :: StartDictionary (Some (1)) , Event :: String ("Cat" . into ()) , Event :: StartDictionary (None) , Event :: String ("age" . into ()) , Event :: Integer (Integer :: from (- 12)) , Event :: String ("name" . into ()) , Event :: String ("Paws" . into ()) , Event :: EndCollection , Event :: EndCollection ,] ; assert_roundtrip (cat , comparison , true) ; }
};
}
