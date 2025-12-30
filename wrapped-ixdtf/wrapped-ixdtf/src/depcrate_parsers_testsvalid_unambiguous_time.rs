// Generated macro for valid_unambiguous_time (function)
macro_rules! Depcrate_parsers_testsvalid_unambiguous_time {
() => {
// Module: crate::parsers::tests
// Provides: {"valid_unambiguous_time"}
// Dependencies: {}
# [test] fn valid_unambiguous_time () { const TIMES : & [& str] = & ["2021-13" , "202113" , "2021-13[-13:00]" , "202113[-13:00]" , "0000-00" , "000000" , "0000-00[UTC]" , "000000[UTC]" , "1314" , "13-14" , "1232" , "0230" , "0631" , "0000" , "00-00" ,] ; for good_value in TIMES { let result = IxdtfParser :: from_str (good_value) . parse_time () ; assert ! (result . is_ok () , "Invalid time parsing: \"{good_value}\" is unambiguous, expected success, got {result:?}") ; } }
};
}
