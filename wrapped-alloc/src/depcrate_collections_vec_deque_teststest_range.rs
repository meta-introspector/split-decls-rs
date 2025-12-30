// Generated macro for test_range (function)
macro_rules! Depcrate_collections_vec_deque_teststest_range {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_range"}
// Dependencies: {}
# [test] fn test_range () { let mut tester : VecDeque < usize > = VecDeque :: with_capacity (7) ; let cap = tester . capacity () ; let minlen = if cfg ! (miri) { cap - 1 } else { 0 } ; for len in minlen ..= cap { for head in 0 ..= cap { for start in 0 ..= len { for end in start ..= len { tester . head = head ; tester . len = 0 ; for i in 0 .. len { tester . push_back (i) ; } let range : VecDeque < _ > = tester . range (start .. end) . copied () . collect () ; let expected : VecDeque < _ > = (start .. end) . collect () ; assert_eq ! (range , expected) ; } } } } }
};
}
