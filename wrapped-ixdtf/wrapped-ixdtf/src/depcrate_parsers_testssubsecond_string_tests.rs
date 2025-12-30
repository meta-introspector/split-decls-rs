// Generated macro for subsecond_string_tests (function)
macro_rules! Depcrate_parsers_testssubsecond_string_tests {
() => {
// Module: crate::parsers::tests
// Provides: {"subsecond_string_tests"}
// Dependencies: {}
# [test] fn subsecond_string_tests () { let hanging_subsecond_start = "15:23:30." ; let err = IxdtfParser :: from_str (hanging_subsecond_start) . parse_time () ; assert ! (err . is_err ()) ; let subsecond_time = "2025-01-15T15:23:30.1" ; let result = IxdtfParser :: from_str (subsecond_time) . parse () ; assert_eq ! (result , Ok (IxdtfParseRecord { date : Some (DateRecord { year : 2025 , month : 1 , day : 15 , }) , time : Some (TimeRecord { hour : 15 , minute : 23 , second : 30 , fraction : Some (Fraction { digits : NonZeroU8 :: new (1) . unwrap () , value : 1 , }) , }) , offset : None , tz : None , calendar : None , })) ; let subsecond_time = "2025-01-15T15:23:30.12345678" ; let result = IxdtfParser :: from_str (subsecond_time) . parse () ; assert_eq ! (result , Ok (IxdtfParseRecord { date : Some (DateRecord { year : 2025 , month : 1 , day : 15 , }) , time : Some (TimeRecord { hour : 15 , minute : 23 , second : 30 , fraction : Some (Fraction { digits : NonZeroU8 :: new (8) . unwrap () , value : 12_345_678 }) , }) , offset : None , tz : None , calendar : None , })) ; let subsecond_time = "2025-01-15T15:23:30.123456789" ; let result = IxdtfParser :: from_str (subsecond_time) . parse () ; assert_eq ! (result , Ok (IxdtfParseRecord { date : Some (DateRecord { year : 2025 , month : 1 , day : 15 , }) , time : Some (TimeRecord { hour : 15 , minute : 23 , second : 30 , fraction : Some (Fraction { digits : NonZeroU8 :: new (9) . unwrap () , value : 123_456_789 , }) , }) , offset : None , tz : None , calendar : None , })) ; }
};
}
