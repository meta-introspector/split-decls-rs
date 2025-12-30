// Generated macro for test_truncate_noops_if_new_len_greater_than_current_static_str (function)
macro_rules! Depcrate_teststest_truncate_noops_if_new_len_greater_than_current_static_str {
() => {
// Module: crate::tests
// Provides: {"test_truncate_noops_if_new_len_greater_than_current_static_str"}
// Dependencies: {}
# [test] fn test_truncate_noops_if_new_len_greater_than_current_static_str () { let mut short = CompactString :: const_new ("short") ; short . truncate (100) ; assert_eq ! (short . len () , 5) ; assert_eq ! (short . capacity () , MAX_SIZE) ; let mut long = CompactString :: const_new ("i am a long string that will be allocated on the heap") ; long . truncate (500) ; assert_eq ! (long . len () , 53) ; assert_eq ! (long . capacity () , 53) ; }
};
}
