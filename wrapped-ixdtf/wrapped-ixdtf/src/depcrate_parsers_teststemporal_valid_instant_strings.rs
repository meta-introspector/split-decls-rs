// Generated macro for temporal_valid_instant_strings (function)
macro_rules! Depcrate_parsers_teststemporal_valid_instant_strings {
() => {
// Module: crate::parsers::tests
// Provides: {"temporal_valid_instant_strings"}
// Dependencies: {}
# [test] fn temporal_valid_instant_strings () { let instants = ["1970-01-01T00:00+00:00[!Africa/Abidjan]" , "1970-01-01T00:00+00:00[UTC]" , "1970-01-01T00:00Z[!Europe/Vienna]" ,] ; for test in instants { let result = IxdtfParser :: from_str (test) . parse () ; assert ! (result . is_ok ()) ; } }
};
}
