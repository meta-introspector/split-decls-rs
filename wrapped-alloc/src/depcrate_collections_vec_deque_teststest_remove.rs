// Generated macro for test_remove (function)
macro_rules! Depcrate_collections_vec_deque_teststest_remove {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_remove"}
// Dependencies: {}
# [test] fn test_remove () { let mut tester = VecDeque :: with_capacity (15) ; let cap = tester . capacity () ; let minlen = if cfg ! (miri) { cap - 2 } else { 0 } ; for len in minlen .. cap - 1 { let expected = (0 ..) . take (len) . collect :: < VecDeque < _ > > () ; for head_pos in 0 .. cap { for to_remove in 0 ..= len { tester . head = head_pos ; tester . len = 0 ; for i in 0 .. len { if i == to_remove { tester . push_back (1234) ; } tester . push_back (i) ; } if to_remove == len { tester . push_back (1234) ; } tester . remove (to_remove) ; assert ! (tester . head <= tester . capacity ()) ; assert ! (tester . len <= tester . capacity ()) ; assert_eq ! (tester , expected) ; } } } }
};
}
