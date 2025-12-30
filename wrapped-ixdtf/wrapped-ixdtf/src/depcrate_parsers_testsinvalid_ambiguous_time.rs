// Generated macro for invalid_ambiguous_time (function)
macro_rules! Depcrate_parsers_testsinvalid_ambiguous_time {
() => {
// Module: crate::parsers::tests
// Provides: {"invalid_ambiguous_time"}
// Dependencies: {}
# [test] fn invalid_ambiguous_time () { const TIMES : & [(& str , ParseError)] = & [("1208-10" , ParseError :: AmbiguousTimeMonthDay) , ("2021-12" , ParseError :: AmbiguousTimeYearMonth) , ("2021-12[-12:00]" , ParseError :: AmbiguousTimeYearMonth) , ("1214" , ParseError :: AmbiguousTimeMonthDay) , ("0229" , ParseError :: AmbiguousTimeMonthDay) , ("1130" , ParseError :: AmbiguousTimeMonthDay) , ("12-14" , ParseError :: AmbiguousTimeMonthDay) , ("12-14[-14:00]" , ParseError :: AmbiguousTimeMonthDay) , ("202112" , ParseError :: AmbiguousTimeYearMonth) , ("202112[UTC]" , ParseError :: AmbiguousTimeYearMonth) ,] ; for (bad_value , error) in TIMES { let result = IxdtfParser :: from_str (bad_value) . parse_time () ; assert_eq ! (result , Err (* error) , "Invalid time parsing: \"{bad_value}\" is ambiguous, expected {error:}, got {result:?}") ; } }
};
}
