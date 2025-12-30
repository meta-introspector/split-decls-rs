// Generated macro for temporal_month_day (function)
macro_rules! Depcrate_parsers_teststemporal_month_day {
() => {
// Module: crate::parsers::tests
// Provides: {"temporal_month_day"}
// Dependencies: {}
# [test] fn temporal_month_day () { let possible_month_day = ["11-07" , "1107[+04:00]" , "--11-07" , "--1107[+04:00]"] ; for md in possible_month_day { let result = IxdtfParser :: from_str (md) . parse_month_day () . unwrap () ; let date = result . date . unwrap () ; assert_eq ! (date . month , 11) ; assert_eq ! (date . day , 7) ; } }
};
}
