// Generated macro for temporal_invalid_durations (function)
macro_rules! Depcrate_parsers_teststemporal_invalid_durations {
() => {
// Module: crate::parsers::tests
// Provides: {"temporal_invalid_durations"}
// Dependencies: {}
# [test] # [cfg (feature = "duration")] fn temporal_invalid_durations () { use crate :: parsers :: IsoDurationParser ; let invalids = ["P1Y1M1W0,5D" , "+PT" , "P1Y1M1W1DT1H0.5M0.5S" , "P" , "PT" , "-P" , "-PT" ,] ; for test in invalids { let err = IsoDurationParser :: from_str (test) . parse () ; assert ! (err . is_err () , "Invalid ISO8601 Duration target: \"{test}\" should fail duration parsing.") ; } }
};
}
