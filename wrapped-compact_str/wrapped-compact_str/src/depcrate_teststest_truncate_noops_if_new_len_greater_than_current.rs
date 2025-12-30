// Generated macro for test_truncate_noops_if_new_len_greater_than_current (function)
macro_rules! Depcrate_teststest_truncate_noops_if_new_len_greater_than_current {
() => {
// Module: crate::tests
// Provides: {"test_truncate_noops_if_new_len_greater_than_current"}
// Dependencies: {}
# [test] fn test_truncate_noops_if_new_len_greater_than_current () { let mut short = CompactString :: from ("short") ; let short_cap = short . capacity () ; short . truncate (100) ; assert_eq ! (short . len () , 5) ; assert_eq ! (short . capacity () , short_cap) ; let mut long = CompactString :: from ("i am a long string that will be allocated on the heap") ; let long_cap = long . capacity () ; long . truncate (500) ; assert_eq ! (long . len () , 53) ; assert_eq ! (long . capacity () , long_cap) ; }
};
}
