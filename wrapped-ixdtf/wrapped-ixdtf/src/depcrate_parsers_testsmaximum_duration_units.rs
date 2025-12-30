// Generated macro for maximum_duration_units (function)
macro_rules! Depcrate_parsers_testsmaximum_duration_units {
() => {
// Module: crate::parsers::tests
// Provides: {"maximum_duration_units"}
// Dependencies: {}
# [test] # [cfg (feature = "duration")] fn maximum_duration_units () { use crate :: parsers :: IsoDurationParser ; let result = IsoDurationParser :: from_str ("P416999965497D") . parse () ; assert ! (result . is_ok ()) ; let result = IsoDurationParser :: from_str ("PT25019997929836H") . parse () ; assert ! (result . is_ok ()) ; let result = IsoDurationParser :: from_str ("PT1501199875790165M") . parse () ; assert ! (result . is_ok ()) ; let result = IsoDurationParser :: from_str ("PT90071992547409910S") . parse () ; assert ! (result . is_ok ()) ; let result = IsoDurationParser :: from_str ("-P416999965497D") . parse () ; assert ! (result . is_ok ()) ; let result = IsoDurationParser :: from_str ("-PT25019997929836H") . parse () ; assert ! (result . is_ok ()) ; let result = IsoDurationParser :: from_str ("-PT1501199875790165M") . parse () ; assert ! (result . is_ok ()) ; let result = IsoDurationParser :: from_str ("-PT90071992547409910S") . parse () ; assert ! (result . is_ok ()) ; }
};
}
