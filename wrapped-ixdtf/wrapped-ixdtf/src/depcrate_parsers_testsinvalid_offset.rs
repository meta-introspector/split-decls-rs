// Generated macro for invalid_offset (function)
macro_rules! Depcrate_parsers_testsinvalid_offset {
() => {
// Module: crate::parsers::tests
// Provides: {"invalid_offset"}
// Dependencies: {}
# [test] fn invalid_offset () { let offset_leap_second = "2024-08-24T14:00:00-05:00:60" ; let err = IxdtfParser :: from_str (offset_leap_second) . parse () ; assert_eq ! (err , Err (ParseError :: TimeMinuteSecond) , "Should fail to parse leap second value.") ; let offset_leap_second = "2024-08-24T14:00:00-050060" ; let err = IxdtfParser :: from_str (offset_leap_second) . parse () ; assert_eq ! (err , Err (ParseError :: TimeMinuteSecond) , "Should fail to parse leap second value.") ; let offset_leap_second = "2024-08-24T14:00:00-05:0060" ; let err = IxdtfParser :: from_str (offset_leap_second) . parse () ; assert_eq ! (err , Err (ParseError :: UtcTimeSeparator) , "Should fail to parse unbalanced time separator") ; let offset_leap_second = "2024-08-24T14:00:00-05:00[-05:00:60]" ; let err = IxdtfParser :: from_str (offset_leap_second) . parse () ; assert_eq ! (err , Err (ParseError :: InvalidMinutePrecisionOffset) , "Should enforce UtcMinutePrecision for annotations") ; let offset_leap_second = "2024-08-24T14:00:00-05:00[-05:00:60]" ; let err = IxdtfParser :: from_str (offset_leap_second) . parse () ; assert_eq ! (err , Err (ParseError :: InvalidMinutePrecisionOffset) , "Should enforce UtcMinutePrecision for annotations") ; }
};
}
