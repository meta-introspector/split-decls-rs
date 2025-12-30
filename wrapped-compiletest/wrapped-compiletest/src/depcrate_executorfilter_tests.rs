// Generated macro for filter_tests (function)
macro_rules! Depcrate_executorfilter_tests {
() => {
// Module: crate::executor
// Provides: {"filter_tests"}
// Dependencies: {}
# [doc = " Applies command-line arguments for filtering/skipping tests by name."] # [doc = ""] # [doc = " Adapted from `filter_tests` in libtest."] # [doc = ""] # [doc = " FIXME(#139660): Now that libtest has been removed, redesign the whole filtering system to"] # [doc = " do a better job of understanding and filtering _paths_, instead of being tied to libtest's"] # [doc = " substring/exact matching behaviour."] fn filter_tests (opts : & Config , tests : Vec < CollectedTest >) -> Vec < CollectedTest > { let mut filtered = tests ; let matches_filter = | test : & CollectedTest , filter_str : & str | { let test_name = & test . desc . name ; if opts . filter_exact { test_name == filter_str } else { test_name . contains (filter_str) } } ; if ! opts . filters . is_empty () { filtered . retain (| test | opts . filters . iter () . any (| filter | matches_filter (test , filter))) ; } if ! opts . skip . is_empty () { filtered . retain (| test | ! opts . skip . iter () . any (| sf | matches_filter (test , sf))) ; } filtered }
};
}
