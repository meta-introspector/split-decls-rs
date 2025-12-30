// Generated macro for deserialize_dictionary_binary (function)
macro_rules! Depcrate_serde_testsdeserialize_dictionary_binary {
() => {
// Module: crate::serde_tests
// Provides: {"deserialize_dictionary_binary"}
// Dependencies: {}
# [test] fn deserialize_dictionary_binary () { let reader = File :: open ("./tests/data/binary.plist") . unwrap () ; let dict : Dictionary = crate :: from_reader (reader) . unwrap () ; check_common_plist (& dict) ; }
};
}
