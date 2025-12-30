// Generated macro for assert_not_contains (function)
macro_rules! Depcrate_assertion_helpersassert_not_contains {
() => {
// Module: crate::assertion_helpers
// Provides: {"assert_not_contains"}
// Dependencies: {}
# [doc = " Assert that `haystack` does not contain `needle`."] # [track_caller] pub fn assert_not_contains < H : AsRef < str > , N : AsRef < str > > (haystack : H , needle : N) { let haystack = haystack . as_ref () ; let needle = needle . as_ref () ; if haystack . contains (needle) { SearchDetails { assertion_name : "assert_not_contains" , haystack , needle } . dump () ; panic ! ("needle was unexpectedly found in haystack") ; } }
};
}
