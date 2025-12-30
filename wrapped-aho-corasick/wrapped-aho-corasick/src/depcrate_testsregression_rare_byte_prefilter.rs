// Generated macro for regression_rare_byte_prefilter (function)
macro_rules! Depcrate_testsregression_rare_byte_prefilter {
() => {
// Module: crate::tests
// Provides: {"regression_rare_byte_prefilter"}
// Dependencies: {}
# [test] fn regression_rare_byte_prefilter () { use crate :: AhoCorasick ; let ac = AhoCorasick :: new (& ["ab/j/" , "x/"]) . unwrap () ; assert ! (ac . is_match ("ab/j/")) ; }
};
}
