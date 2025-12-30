// Generated macro for test_insert (function)
macro_rules! Depcrate_collections_vec_deque_teststest_insert {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_insert"}
// Dependencies: {}
# [test] fn test_insert () { let mut tester = VecDeque :: with_capacity (15) ; let cap = tester . capacity () ; let minlen = if cfg ! (miri) { cap - 1 } else { 1 } ; for len in minlen .. cap { let expected = (0 ..) . take (len) . collect :: < VecDeque < _ > > () ; for head_pos in 0 .. cap { for to_insert in 0 .. len { tester . head = head_pos ; tester . len = 0 ; for i in 0 .. len { if i != to_insert { tester . push_back (i) ; } } tester . insert (to_insert , to_insert) ; assert ! (tester . head <= tester . capacity ()) ; assert ! (tester . len <= tester . capacity ()) ; assert_eq ! (tester , expected) ; } } } }
};
}
