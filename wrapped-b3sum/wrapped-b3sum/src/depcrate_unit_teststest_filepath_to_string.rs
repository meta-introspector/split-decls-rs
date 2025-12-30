// Generated macro for test_filepath_to_string (function)
macro_rules! Depcrate_unit_teststest_filepath_to_string {
() => {
// Module: crate::unit_tests
// Provides: {"test_filepath_to_string"}
// Dependencies: {}
# [test] fn test_filepath_to_string () { let output = crate :: filepath_to_string (Path :: new ("foo")) ; assert_eq ! (output . filepath_string , "foo") ; assert ! (! output . is_escaped) ; let output = crate :: filepath_to_string (Path :: new ("f\\ \t\r\noo")) ; if cfg ! (windows) { assert_eq ! (output . filepath_string , "f/ \t\\r\\noo") ; } else { assert_eq ! (output . filepath_string , "f\\\\ \t\\r\\noo") ; } assert ! (output . is_escaped) ; }
};
}
