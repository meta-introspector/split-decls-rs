// Generated macro for subseconds_parsing_extended_femtoseconds (function)
macro_rules! Depcrate_parsers_testssubseconds_parsing_extended_femtoseconds {
() => {
// Module: crate::parsers::tests
// Provides: {"subseconds_parsing_extended_femtoseconds"}
// Dependencies: {}
# [test] fn subseconds_parsing_extended_femtoseconds () { let subsecond_time = "15:23:30.1234567898765" ; let result = IxdtfParser :: from_str (subsecond_time) . parse_time () ; assert_eq ! (result , Ok (IxdtfParseRecord { date : None , time : Some (TimeRecord { hour : 15 , minute : 23 , second : 30 , fraction : Some (Fraction { digits : NonZeroU8 :: new (13) . unwrap () , value : 1_234_567_898_765 , }) }) , offset : None , tz : None , calendar : None , })) ; let subsecond_time = "15:23:30.123456789876543" ; let result = IxdtfParser :: from_str (subsecond_time) . parse_time () ; assert_eq ! (result , Ok (IxdtfParseRecord { date : None , time : Some (TimeRecord { hour : 15 , minute : 23 , second : 30 , fraction : Some (Fraction { digits : NonZeroU8 :: new (15) . unwrap () , value : 123_456_789_876_543 }) }) , offset : None , tz : None , calendar : None , })) ; }
};
}
