// Generated macro for prefilter_stays_in_bounds (function)
macro_rules! Depcrate_testsprefilter_stays_in_bounds {
() => {
// Module: crate::tests
// Provides: {"prefilter_stays_in_bounds"}
// Dependencies: {}
# [test] fn prefilter_stays_in_bounds () { let ac = AhoCorasick :: builder () . match_kind (MatchKind :: LeftmostFirst) . build (& ["sam" , "frodo" , "pippin" , "merry" , "gandalf" , "sauron"]) . unwrap () ; let haystack = "foo gandalf" ; assert_eq ! (None , ac . find (Input :: new (haystack) . range (0 .. 10))) ; }
};
}
