// Generated macro for duration_exceeds_range (function)
macro_rules! Depcrate_parsers_testsduration_exceeds_range {
() => {
// Module: crate::parsers::tests
// Provides: {"duration_exceeds_range"}
// Dependencies: {}
# [test] # [cfg (feature = "duration")] fn duration_exceeds_range () { use crate :: parsers :: IsoDurationParser ; let test = "P1000000000000000000000000000000000000000YT1H" ; let err = IsoDurationParser :: from_str (test) . parse () ; assert_eq ! (err , Err (ParseError :: DurationValueExceededRange)) ; let test = "P1YT1000000000000000000000000000000000000000H" ; let err = IsoDurationParser :: from_str (test) . parse () ; assert_eq ! (err , Err (ParseError :: DurationValueExceededRange)) ; }
};
}
