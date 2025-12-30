// Generated macro for regression_case_insensitive_prefilter (function)
macro_rules! Depcrate_testsregression_case_insensitive_prefilter {
() => {
// Module: crate::tests
// Provides: {"regression_case_insensitive_prefilter"}
// Dependencies: {}
# [test] fn regression_case_insensitive_prefilter () { for c in b'a' .. b'z' { for c2 in b'a' .. b'z' { let c = c as char ; let c2 = c2 as char ; let needle = format ! ("{}{}" , c , c2) . to_lowercase () ; let haystack = needle . to_uppercase () ; let ac = AhoCorasick :: builder () . ascii_case_insensitive (true) . prefilter (true) . build (& [& needle]) . unwrap () ; assert_eq ! (1 , ac . find_iter (& haystack) . count () , "failed to find {:?} in {:?}\n\nautomaton:\n{:?}" , needle , haystack , ac ,) ; } } }
};
}
