// Generated macro for test_string (function)
macro_rules! Depcrate_de_teststest_string {
() => {
// Module: crate::de::tests
// Provides: {"test_string"}
// Dependencies: {}
# [test] fn test_string () { check_from_str_bytes_reader ("\"String\"" , Ok (String :: from ("String"))) ; check_from_str_bytes_reader ("r\"String\"" , Ok (String :: from ("String"))) ; check_from_str_bytes_reader ("r#\"String\"#" , Ok (String :: from ("String"))) ; check_from_str_bytes_reader ("r#\"String with\nmultiple\nlines\n\"#" , Ok (String :: from ("String with\nmultiple\nlines\n")) ,) ; check_from_str_bytes_reader ("r##\"String with \"#\"##" , Ok (String :: from ("String with \"#")) ,) ; }
};
}
