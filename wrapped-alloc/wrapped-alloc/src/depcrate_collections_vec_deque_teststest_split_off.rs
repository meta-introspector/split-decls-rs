// Generated macro for test_split_off (function)
macro_rules! Depcrate_collections_vec_deque_teststest_split_off {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_split_off"}
// Dependencies: {}
# [test] fn test_split_off () { let mut tester = VecDeque :: with_capacity (15) ; let cap = tester . capacity () ; let minlen = if cfg ! (miri) { cap - 1 } else { 0 } ; for len in minlen .. cap { for at in 0 ..= len { let expected_self = (0 ..) . take (at) . collect :: < VecDeque < _ > > () ; let expected_other = (at ..) . take (len - at) . collect :: < VecDeque < _ > > () ; for head_pos in 0 .. cap { tester . head = head_pos ; tester . len = 0 ; for i in 0 .. len { tester . push_back (i) ; } let result = tester . split_off (at) ; assert ! (tester . head <= tester . capacity ()) ; assert ! (tester . len <= tester . capacity ()) ; assert ! (result . head <= result . capacity ()) ; assert ! (result . len <= result . capacity ()) ; assert_eq ! (tester , expected_self) ; assert_eq ! (result , expected_other) ; } } } }
};
}
