// Generated macro for subseconds_parsing_extended_picoseconds (function)
macro_rules! Depcrate_parsers_testssubseconds_parsing_extended_picoseconds {
() => {
// Module: crate::parsers::tests
// Provides: {"subseconds_parsing_extended_picoseconds"}
// Dependencies: {}
# [test] fn subseconds_parsing_extended_picoseconds () { let subsecond_time = "15:23:30.1234567890" ; let result = IxdtfParser :: from_str (subsecond_time) . parse_time () ; assert_eq ! (result , Ok (IxdtfParseRecord { date : None , time : Some (TimeRecord { hour : 15 , minute : 23 , second : 30 , fraction : Some (Fraction { digits : NonZeroU8 :: new (10) . unwrap () , value : 1_234_567_890 , }) }) , offset : None , tz : None , calendar : None , })) ; let subsecond_time = "15:23:30.123456789876" ; let result = IxdtfParser :: from_str (subsecond_time) . parse_time () ; assert_eq ! (result , Ok (IxdtfParseRecord { date : None , time : Some (TimeRecord { hour : 15 , minute : 23 , second : 30 , fraction : Some (Fraction { digits : NonZeroU8 :: new (12) . unwrap () , value : 123_456_789_876 , }) }) , offset : None , tz : None , calendar : None , })) ; }
};
}
