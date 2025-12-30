// Generated macro for temporal_date_time_max (function)
macro_rules! Depcrate_parsers_teststemporal_date_time_max {
() => {
// Module: crate::parsers::tests
// Provides: {"temporal_date_time_max"}
// Dependencies: {}
# [test] fn temporal_date_time_max () { let result = IxdtfParser :: from_str ("+002020-11-08T12:28:32.329402834[!America/Argentina/ComodRivadavia][!u-ca=iso8601]" ,) . parse () . unwrap () ; assert_eq ! (result . time , Some (TimeRecord { hour : 12 , minute : 28 , second : 32 , fraction : Some (Fraction { digits : NonZeroU8 :: new (9) . unwrap () , value : 329402834 }) })) ; }
};
}
