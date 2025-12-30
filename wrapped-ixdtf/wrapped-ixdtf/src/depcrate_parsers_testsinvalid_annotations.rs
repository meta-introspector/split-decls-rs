// Generated macro for invalid_annotations (function)
macro_rules! Depcrate_parsers_testsinvalid_annotations {
() => {
// Module: crate::parsers::tests
// Provides: {"invalid_annotations"}
// Dependencies: {}
# [test] fn invalid_annotations () { let bad_value = "2021-01-29 02:12:48+01:00:00(u-ca=iso8601]" ; let err = IxdtfParser :: from_str (bad_value) . parse () ; assert_eq ! (err , Err (ParseError :: InvalidEnd) , "Invalid annotation parsing: \"{bad_value}\" should fail to parse.") ; let bad_value = "2021-01-29 02:12:48+01:00:00[u-ca=iso8601)" ; let err = IxdtfParser :: from_str (bad_value) . parse () ; assert_eq ! (err , Err (ParseError :: AnnotationValueChar) , "Invalid annotation parsing: \"{bad_value}\" should fail to parse.") ; let bad_value = "2021-01-29 02:12:48+01:00:00[u-ca=iso8601][!foo=bar]" ; let err = IxdtfParser :: from_str (bad_value) . parse () ; assert_eq ! (err , Err (ParseError :: UnrecognizedCritical) , "Invalid annotation parsing: \"{bad_value}\" should fail to parse.") ; }
};
}
