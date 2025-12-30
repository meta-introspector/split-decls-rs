// Generated macro for temporal_parser_basic (function)
macro_rules! Depcrate_parsers_teststemporal_parser_basic {
() => {
// Module: crate::parsers::tests
// Provides: {"temporal_parser_basic"}
// Dependencies: {}
# [test] fn temporal_parser_basic () { let basic_result = IxdtfParser :: from_str ("20201108") . parse () . unwrap () ; let sep_result = IxdtfParser :: from_str ("2020-11-08") . parse () . unwrap () ; assert_eq ! (basic_result . date , Some (DateRecord { year : 2020 , month : 11 , day : 8 , })) ; assert_eq ! (basic_result . date , sep_result . date ,) ; }
};
}
