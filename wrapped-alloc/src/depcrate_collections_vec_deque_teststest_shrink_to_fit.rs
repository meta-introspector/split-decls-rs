// Generated macro for test_shrink_to_fit (function)
macro_rules! Depcrate_collections_vec_deque_teststest_shrink_to_fit {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_shrink_to_fit"}
// Dependencies: {}
# [test] fn test_shrink_to_fit () { let mut tester = VecDeque :: with_capacity (15) ; let cap = tester . capacity () ; tester . reserve (63) ; let max_cap = tester . capacity () ; for len in 0 ..= cap { let expected = (0 ..) . take (len) . collect :: < VecDeque < _ > > () ; for head_pos in 0 ..= max_cap { tester . reserve (head_pos) ; tester . head = head_pos ; tester . len = 0 ; tester . reserve (63) ; for i in 0 .. len { tester . push_back (i) ; } tester . shrink_to_fit () ; assert ! (tester . capacity () <= cap) ; assert ! (tester . head <= tester . capacity ()) ; assert ! (tester . len <= tester . capacity ()) ; assert_eq ! (tester , expected) ; } } }
};
}
