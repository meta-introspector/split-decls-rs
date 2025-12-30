// Generated macro for regression_ascii_case_insensitive_no_exponential (function)
macro_rules! Depcrate_testsregression_ascii_case_insensitive_no_exponential {
() => {
// Module: crate::tests
// Provides: {"regression_ascii_case_insensitive_no_exponential"}
// Dependencies: {}
# [test] fn regression_ascii_case_insensitive_no_exponential () { let ac = AhoCorasick :: builder () . ascii_case_insensitive (true) . build (& ["Tsubaki House-Triple Shot Vol01校花三姐妹"]) . unwrap () ; assert ! (ac . find ("") . is_none ()) ; }
};
}
