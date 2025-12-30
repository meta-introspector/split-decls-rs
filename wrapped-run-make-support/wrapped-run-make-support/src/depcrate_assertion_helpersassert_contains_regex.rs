// Generated macro for assert_contains_regex (function)
macro_rules! Depcrate_assertion_helpersassert_contains_regex {
() => {
// Module: crate::assertion_helpers
// Provides: {"assert_contains_regex"}
// Dependencies: {}
# [doc = " Assert that `haystack` contains the regex `needle`."] # [track_caller] pub fn assert_contains_regex < H : AsRef < str > , N : AsRef < str > > (haystack : H , needle : N) { let haystack = haystack . as_ref () ; let needle = needle . as_ref () ; let re = regex :: Regex :: new (needle) . unwrap () ; if ! re . is_match (haystack) { SearchDetails { assertion_name : "assert_contains_regex" , haystack , needle } . dump () ; panic ! ("regex was not found in haystack") ; } }
};
}
