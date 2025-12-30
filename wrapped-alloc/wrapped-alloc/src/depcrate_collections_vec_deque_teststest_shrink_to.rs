// Generated macro for test_shrink_to (function)
macro_rules! Depcrate_collections_vec_deque_teststest_shrink_to {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_shrink_to"}
// Dependencies: {}
# [test] fn test_shrink_to () { let cap = 16 ; for len in 0 .. cap { for head in 0 .. cap { let expected = (1 ..= len) . collect :: < VecDeque < _ > > () ; for target_cap in len .. cap { let mut deque = VecDeque :: with_capacity (cap) ; assert_eq ! (deque . capacity () , cap) ; deque . head = head ; deque . extend (1 ..= len) ; deque . shrink_to (target_cap) ; assert_eq ! (deque , expected) ; } } } }
};
}
