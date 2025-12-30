// Generated macro for test_offset_annotation (function)
macro_rules! Depcrate_parsers_teststest_offset_annotation {
() => {
// Module: crate::parsers::tests
// Provides: {"test_offset_annotation"}
// Dependencies: {}
# [test] fn test_offset_annotation () { let zdt = "2024-08-24T14:00:00-05:00[-05:00.123456789]" ; let err = IxdtfParser :: from_str (zdt) . parse () ; assert_eq ! (err , Err (ParseError :: AnnotationClose)) ; let zdt = "2024-08-24T14:00:00-05:00[-05:00.123456]" ; let err = IxdtfParser :: from_str (zdt) . parse () ; assert_eq ! (err , Err (ParseError :: AnnotationClose)) ; let zdt = "2024-08-24T14:00:00-05:00[-05:00.1]" ; let err = IxdtfParser :: from_str (zdt) . parse () ; assert_eq ! (err , Err (ParseError :: AnnotationClose)) ; }
};
}
