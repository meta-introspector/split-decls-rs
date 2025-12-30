// Generated macro for maximum_duration_fraction (function)
macro_rules! Depcrate_parsers_testsmaximum_duration_fraction {
() => {
// Module: crate::parsers::tests
// Provides: {"maximum_duration_fraction"}
// Dependencies: {}
# [test] # [cfg (feature = "duration")] fn maximum_duration_fraction () { use crate :: parsers :: IsoDurationParser ; let test = "P1Y1DT1.999999999H" ; let result = IsoDurationParser :: from_str (test) . parse () ; assert ! (result . is_ok ()) ; let test = "P1Y1DT1H1.999999999M" ; let result = IsoDurationParser :: from_str (test) . parse () ; assert ! (result . is_ok ()) ; let test = "P1Y1DT1H1M1.999999999S" ; let result = IsoDurationParser :: from_str (test) . parse () ; assert ! (result . is_ok ()) ; }
};
}
