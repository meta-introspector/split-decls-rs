// Generated macro for invalid_month_day (function)
macro_rules! Depcrate_parsers_testsinvalid_month_day {
() => {
// Module: crate::parsers::tests
// Provides: {"invalid_month_day"}
// Dependencies: {}
# [test] fn invalid_month_day () { let bad_value = "-11-07" ; let err = IxdtfParser :: from_str (bad_value) . parse_month_day () ; assert_eq ! (err , Err (ParseError :: MonthDayHyphen)) }
};
}
