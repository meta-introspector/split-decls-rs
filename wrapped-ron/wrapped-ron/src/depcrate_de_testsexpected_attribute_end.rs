// Generated macro for expected_attribute_end (function)
macro_rules! Depcrate_de_testsexpected_attribute_end {
() => {
// Module: crate::de::tests
// Provides: {"expected_attribute_end"}
// Dependencies: {}
# [test] fn expected_attribute_end () { let bogus_struct = "#![enable(unwrap_newtypes) \"Hello\"" ; let expected_err = err (Error :: ExpectedAttributeEnd , (1 , 27) , (1 , 28)) ; check_from_str_bytes_reader :: < String > (bogus_struct , expected_err . clone ()) ; # [cfg (feature = "internal-span-substring-test")] check_error_span_inclusive :: < String > (bogus_struct , expected_err , " \"") ; }
};
}
