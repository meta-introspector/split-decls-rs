// Generated macro for deserialize_dictionary_xml (function)
macro_rules! Depcrate_serde_testsdeserialize_dictionary_xml {
() => {
// Module: crate::serde_tests
// Provides: {"deserialize_dictionary_xml"}
// Dependencies: {}
# [test] fn deserialize_dictionary_xml () { let reader = File :: open ("./tests/data/xml.plist") . unwrap () ; let dict : Dictionary = crate :: from_reader (reader) . unwrap () ; check_common_plist (& dict) ; assert_eq ! (dict . get ("HexademicalNumber") . unwrap () . as_unsigned_integer () . unwrap () , 0xDEADBEEF) ; }
};
}
