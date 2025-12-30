// Generated macro for test_drain (function)
macro_rules! Depcrate_collections_vec_deque_teststest_drain {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_drain"}
// Dependencies: {}
# [test] fn test_drain () { let mut tester : VecDeque < usize > = VecDeque :: with_capacity (7) ; let cap = tester . capacity () ; for len in 0 ..= cap { for head in 0 .. cap { for drain_start in 0 ..= len { for drain_end in drain_start ..= len { tester . head = head ; tester . len = 0 ; for i in 0 .. len { tester . push_back (i) ; } let drained : VecDeque < _ > = tester . drain (drain_start .. drain_end) . collect () ; let drained_expected : VecDeque < _ > = (drain_start .. drain_end) . collect () ; assert_eq ! (drained , drained_expected) ; assert_eq ! (tester . capacity () , cap) ; assert ! (tester . head <= tester . capacity ()) ; assert ! (tester . len <= tester . capacity ()) ; let expected : VecDeque < _ > = (0 .. drain_start) . chain (drain_end .. len) . collect () ; assert_eq ! (expected , tester) ; } } } } }
};
}
