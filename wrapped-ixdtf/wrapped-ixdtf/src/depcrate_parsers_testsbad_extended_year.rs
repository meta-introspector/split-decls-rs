// Generated macro for bad_extended_year (function)
macro_rules! Depcrate_parsers_testsbad_extended_year {
() => {
// Module: crate::parsers::tests
// Provides: {"bad_extended_year"}
// Dependencies: {}
# [test] fn bad_extended_year () { let bad_year = "-000000-11-08" ; let err = IxdtfParser :: from_str (bad_year) . parse () ; assert_eq ! (err , Err (ParseError :: DateExtendedYear) , "Invalid extended year parsing: \"{bad_year}\" should fail to parse.") ; let bad_year = "-1000000-11-08" ; let err = IxdtfParser :: from_str (bad_year) . parse () ; assert_eq ! (err , Err (ParseError :: DateMonth) , "Invalid year range parsing: \"{bad_year}\" should fail to parse.") ; let bad_year = "+10000001108" ; let err = IxdtfParser :: from_str (bad_year) . parse () ; assert_eq ! (err , Err (ParseError :: InvalidEnd) , "Invalid year range parsing: \"{bad_year}\" should fail to parse.") ; }
};
}
