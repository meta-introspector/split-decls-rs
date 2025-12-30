// Generated macro for temporal_year_month (function)
macro_rules! Depcrate_parsers_teststemporal_year_month {
() => {
// Module: crate::parsers::tests
// Provides: {"temporal_year_month"}
// Dependencies: {}
# [test] fn temporal_year_month () { let possible_year_months = ["+002020-11" , "2020-11[u-ca=iso8601]" , "+00202011" , "202011[u-ca=iso8601]" ,] ; for ym in possible_year_months { let result = IxdtfParser :: from_str (ym) . parse_year_month () . unwrap () ; let date = result . date . unwrap () ; assert_eq ! (date . year , 2020) ; assert_eq ! (date . month , 11) ; } }
};
}
