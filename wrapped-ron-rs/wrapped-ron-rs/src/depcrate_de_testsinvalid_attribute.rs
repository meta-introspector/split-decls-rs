// Generated macro for invalid_attribute (function)
macro_rules! Depcrate_de_testsinvalid_attribute {
() => {
// Module: crate::de::tests
// Provides: {"invalid_attribute"}
// Dependencies: {}
# [test] fn invalid_attribute () { let bogus_struct = "#![enable(invalid)] \"Hello\"" ; let expected_err = err (Error :: NoSuchExtension ("invalid" . to_string ()) , (1 , 11) , (1 , 18) ,) ; check_from_str_bytes_reader :: < String > (bogus_struct , expected_err . clone ()) ; # [cfg (feature = "internal-span-substring-test")] check_error_span_exclusive :: < String > (bogus_struct , expected_err , "invalid") ; }
};
}
