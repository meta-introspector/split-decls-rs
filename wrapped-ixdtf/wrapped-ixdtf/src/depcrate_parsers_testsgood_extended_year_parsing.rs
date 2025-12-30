// Generated macro for good_extended_year_parsing (function)
macro_rules! Depcrate_parsers_testsgood_extended_year_parsing {
() => {
// Module: crate::parsers::tests
// Provides: {"good_extended_year_parsing"}
// Dependencies: {}
# [test] fn good_extended_year_parsing () { let extended_year = "+002020-11-08" ; let result = IxdtfParser :: from_str (extended_year) . parse () . unwrap () ; assert_eq ! (result . date , Some (DateRecord { year : 2020 , month : 11 , day : 8 , }) , "Extended year \"{extended_year}\" should pass.") ; let extended_year = "-002020-11-08" ; let result = IxdtfParser :: from_str (extended_year) . parse () . unwrap () ; assert_eq ! (result . date , Some (DateRecord { year : - 2020 , month : 11 , day : 8 , }) , "Extended year \"{extended_year}\" should pass.") ; }
};
}
