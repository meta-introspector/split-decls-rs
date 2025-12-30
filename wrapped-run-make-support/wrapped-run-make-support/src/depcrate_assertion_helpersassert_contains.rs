// Generated macro for assert_contains (function)
macro_rules! Depcrate_assertion_helpersassert_contains {
() => {
// Module: crate::assertion_helpers
// Provides: {"assert_contains"}
// Dependencies: {}
# [doc = " Assert that `haystack` contains `needle`."] # [track_caller] pub fn assert_contains < H : AsRef < str > , N : AsRef < str > > (haystack : H , needle : N) { let haystack = haystack . as_ref () ; let needle = needle . as_ref () ; if ! haystack . contains (needle) { SearchDetails { assertion_name : "assert_contains" , haystack , needle } . dump () ; panic ! ("needle was not found in haystack") ; } }
};
}
