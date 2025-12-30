// Generated macro for issue_108453 (function)
macro_rules! Depcrate_collections_vec_deque_testsissue_108453 {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"issue_108453"}
// Dependencies: {}
# [test] fn issue_108453 () { let mut deque = VecDeque :: with_capacity (10) ; deque . push_back (1u8) ; deque . push_back (2) ; deque . push_back (3) ; deque . push_front (10) ; deque . push_front (9) ; deque . shrink_to (9) ; assert_eq ! (deque . into_iter () . collect ::< Vec < _ >> () , vec ! [9 , 10 , 1 , 2 , 3]) ; }
};
}
