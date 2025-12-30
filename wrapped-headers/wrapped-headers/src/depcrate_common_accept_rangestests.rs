// Generated macro for tests (module)
macro_rules! Depcrate_common_accept_rangestests {
() => {
// Module: crate::common::accept_ranges
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: super :: test_decode ; use super :: * ; fn accept_ranges (s : & str) -> AcceptRanges { test_decode (& [s]) . unwrap () } # [test] fn bytes_constructor () { assert_eq ! (accept_ranges ("bytes") , AcceptRanges :: bytes ()) ; } # [test] fn is_bytes_method_successful_with_bytes_ranges () { assert ! (accept_ranges ("bytes") . is_bytes ()) ; } # [test] fn is_bytes_method_successful_with_bytes_ranges_by_constructor () { assert ! (AcceptRanges :: bytes () . is_bytes ()) ; } # [test] fn is_bytes_method_failed_with_not_bytes_ranges () { assert ! (! accept_ranges ("dummy") . is_bytes ()) ; } # [test] fn none_constructor () { assert_eq ! (accept_ranges ("none") , AcceptRanges :: none ()) ; } # [test] fn is_none_method_successful_with_none_ranges () { assert ! (accept_ranges ("none") . is_none ()) ; } # [test] fn is_none_method_successful_with_none_ranges_by_constructor () { assert ! (AcceptRanges :: none () . is_none ()) ; } # [test] fn is_none_method_failed_with_not_none_ranges () { assert ! (! accept_ranges ("dummy") . is_none ()) ; } }
};
}
