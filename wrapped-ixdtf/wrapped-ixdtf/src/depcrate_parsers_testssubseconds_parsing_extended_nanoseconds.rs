// Generated macro for subseconds_parsing_extended_nanoseconds (function)
macro_rules! Depcrate_parsers_testssubseconds_parsing_extended_nanoseconds {
() => {
// Module: crate::parsers::tests
// Provides: {"subseconds_parsing_extended_nanoseconds"}
// Dependencies: {}
# [test] fn subseconds_parsing_extended_nanoseconds () { let subsecond_time = "15:23:30.1234567" ; let result = IxdtfParser :: from_str (subsecond_time) . parse_time () ; assert_eq ! (result , Ok (IxdtfParseRecord { date : None , time : Some (TimeRecord { hour : 15 , minute : 23 , second : 30 , fraction : Some (Fraction { digits : NonZeroU8 :: new (7) . unwrap () , value : 1_234_567 }) , }) , offset : None , tz : None , calendar : None , })) ; let subsecond_time = "15:23:30.123456789" ; let result = IxdtfParser :: from_str (subsecond_time) . parse_time () ; assert_eq ! (result , Ok (IxdtfParseRecord { date : None , time : Some (TimeRecord { hour : 15 , minute : 23 , second : 30 , fraction : Some (Fraction { digits : NonZeroU8 :: new (9) . unwrap () , value : 123_456_789 , }) , }) , offset : None , tz : None , calendar : None , })) ; }
};
}
