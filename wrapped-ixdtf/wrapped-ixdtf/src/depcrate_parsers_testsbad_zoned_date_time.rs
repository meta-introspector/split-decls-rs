// Generated macro for bad_zoned_date_time (function)
macro_rules! Depcrate_parsers_testsbad_zoned_date_time {
() => {
// Module: crate::parsers::tests
// Provides: {"bad_zoned_date_time"}
// Dependencies: {}
# [test] fn bad_zoned_date_time () { let bad_value = "2020-04-08(America/Chicago]" ; let err = IxdtfParser :: from_str (bad_value) . parse () ; assert_eq ! (err , Err (ParseError :: InvalidEnd) , "Invalid ZonedDateTime parsing: \"{bad_value}\" should fail to parse.") ; let bad_value = "2020-04-08[America/Chicago)" ; let err = IxdtfParser :: from_str (bad_value) . parse () ; assert_eq ! (err , Err (ParseError :: AnnotationClose) , "Invalid ZonedDateTime parsing: \"{bad_value}\" should fail to parse.") ; let bad_value = "2020-04-08[America/ Chicago)" ; let err = IxdtfParser :: from_str (bad_value) . parse () ; assert_eq ! (err , Err (ParseError :: IanaCharPostSeparator) , "Invalid ZonedDateTime parsing: \"{bad_value}\" should fail to parse.") ; let bad_value = "2020-04-08[Amer" ; let err = IxdtfParser :: from_str (bad_value) . parse () ; assert_eq ! (err , Err (ParseError :: AnnotationClose) , "Invalid ZonedDateTime parsing: \"{bad_value}\" should fail to parse.") ; let bad_value = "2020-04-08[u-ca=iso8601][Europe/London]" ; let err = IxdtfParser :: from_str (bad_value) . parse () ; assert_eq ! (err , Err (ParseError :: AnnotationKeyLeadingChar) , "Invalid ZonedDateTime parsing: \"{bad_value}\" should fail to parse.") ; }
};
}
