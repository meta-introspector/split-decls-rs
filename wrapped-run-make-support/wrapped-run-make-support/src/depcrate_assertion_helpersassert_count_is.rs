// Generated macro for assert_count_is (function)
macro_rules! Depcrate_assertion_helpersassert_count_is {
() => {
// Module: crate::assertion_helpers
// Provides: {"assert_count_is"}
// Dependencies: {}
# [doc = " Assert that `haystack` contains regex `needle` an `expected_count` number of times."] # [track_caller] pub fn assert_count_is < H : AsRef < str > , N : AsRef < str > > (expected_count : usize , haystack : H , needle : N ,) { let haystack = haystack . as_ref () ; let needle = needle . as_ref () ; let actual_count = haystack . matches (needle) . count () ; if expected_count != actual_count { let count_fmt = format ! ("assert_count_is (expected_count = {expected_count}, actual_count = {actual_count})") ; SearchDetails { assertion_name : & count_fmt , haystack , needle } . dump () ; panic ! ("regex did not appear {expected_count} times in haystack (expected_count = \
            {expected_count}, actual_count = {actual_count})") ; } }
};
}
