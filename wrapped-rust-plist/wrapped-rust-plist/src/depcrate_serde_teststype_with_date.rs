// Generated macro for type_with_date (function)
macro_rules! Depcrate_serde_teststype_with_date {
() => {
// Module: crate::serde_tests
// Provides: {"type_with_date"}
// Dependencies: {}
# [test] fn type_with_date () { let date = Date :: from_xml_format ("1920-01-01T00:10:00Z") . unwrap () ; let obj = TypeWithDate { a : Some (28) , b : Some (date) , } ; let comparison = & [Event :: StartDictionary (None) , Event :: String ("a" . into ()) , Event :: Integer (28 . into ()) , Event :: String ("b" . into ()) , Event :: Date (date) , Event :: EndCollection ,] ; assert_roundtrip (obj , comparison , true) ; }
};
}
