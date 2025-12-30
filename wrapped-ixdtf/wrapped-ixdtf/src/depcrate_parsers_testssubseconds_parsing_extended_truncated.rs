// Generated macro for subseconds_parsing_extended_truncated (function)
macro_rules! Depcrate_parsers_testssubseconds_parsing_extended_truncated {
() => {
// Module: crate::parsers::tests
// Provides: {"subseconds_parsing_extended_truncated"}
// Dependencies: {}
# [test] fn subseconds_parsing_extended_truncated () { let subsecond_time = "15:23:30.1234567898765432" ; let result = IxdtfParser :: from_str (subsecond_time) . parse_time () ; assert_eq ! (result , Ok (IxdtfParseRecord { date : None , time : Some (TimeRecord { hour : 15 , minute : 23 , second : 30 , fraction : Some (Fraction { digits : NonZeroU8 :: new (16) . unwrap () , value : 1_234_567_898_765_432 }) , }) , offset : None , tz : None , calendar : None , })) ; let subsecond_time = "15:23:30.1234567898765432101234567890987654321" ; let result = IxdtfParser :: from_str (subsecond_time) . parse_time () ; assert_eq ! (result , Ok (IxdtfParseRecord { date : None , time : Some (TimeRecord { hour : 15 , minute : 23 , second : 30 , fraction : Some (Fraction { digits : NonZeroU8 :: new (37) . unwrap () , value : 123_456_789_876_543_210 , }) , }) , offset : None , tz : None , calendar : None , })) ; }
};
}
